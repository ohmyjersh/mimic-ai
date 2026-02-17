import * as vscode from "vscode";
import * as https from "https";
import * as fs from "fs";
import * as path from "path";
import * as os from "os";
import { execFile } from "child_process";

interface GitHubAsset {
  name: string;
  browser_download_url: string;
}

interface GitHubRelease {
  tag_name: string;
  assets: GitHubAsset[];
}

/**
 * Map platform/arch to the expected GitHub release asset name.
 */
function getAssetName(): string | undefined {
  const platform = os.platform();
  const arch = os.arch();

  if (platform === "darwin" && arch === "arm64") {
    return "mimic-aarch64-apple-darwin";
  }
  if (platform === "darwin" && arch === "x64") {
    return "mimic-x86_64-apple-darwin";
  }
  if (platform === "linux" && arch === "x64") {
    return "mimic-x86_64-unknown-linux-gnu";
  }
  if (platform === "win32" && arch === "x64") {
    return "mimic-x86_64-pc-windows-msvc.exe";
  }
  return undefined;
}

/**
 * Check if a binary exists and responds to --version.
 */
function canExecute(binaryPath: string): Promise<boolean> {
  return new Promise((resolve) => {
    execFile(binaryPath, ["--version"], (err) => {
      resolve(!err);
    });
  });
}

/**
 * Well-known locations to check for the mimic binary beyond PATH.
 */
function getWellKnownPaths(): string[] {
  const home = os.homedir();
  const paths = [path.join(home, ".cargo", "bin", "mimic")];
  if (os.platform() !== "win32") {
    paths.push("/usr/local/bin/mimic", "/opt/homebrew/bin/mimic");
  }
  return paths;
}

/**
 * Download a URL, following redirects (GitHub redirects to CDN).
 */
function downloadFile(url: string, dest: string): Promise<void> {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    const request = (downloadUrl: string) => {
      https
        .get(downloadUrl, { headers: { "User-Agent": "vscode-mimic" } }, (res) => {
          if (res.statusCode === 302 || res.statusCode === 301) {
            const redirectUrl = res.headers.location;
            if (redirectUrl) {
              request(redirectUrl);
              return;
            }
          }
          if (res.statusCode !== 200) {
            reject(new Error(`Download failed with status ${res.statusCode}`));
            return;
          }
          res.pipe(file);
          file.on("finish", () => {
            file.close(() => resolve());
          });
        })
        .on("error", (err) => {
          fs.unlink(dest, () => {});
          reject(err);
        });
    };
    request(url);
  });
}

/**
 * Fetch the latest release metadata from GitHub.
 */
function fetchLatestRelease(): Promise<GitHubRelease> {
  return new Promise((resolve, reject) => {
    const request = (url: string) => {
      https
        .get(url, { headers: { "User-Agent": "vscode-mimic" } }, (res) => {
          if (res.statusCode === 302 || res.statusCode === 301) {
            const redirectUrl = res.headers.location;
            if (redirectUrl) {
              request(redirectUrl);
              return;
            }
          }
          if (res.statusCode !== 200) {
            reject(new Error(`GitHub API returned status ${res.statusCode}`));
            return;
          }
          let body = "";
          res.on("data", (chunk: Buffer) => {
            body += chunk.toString();
          });
          res.on("end", () => {
            try {
              resolve(JSON.parse(body) as GitHubRelease);
            } catch (err) {
              reject(new Error("Failed to parse GitHub release JSON"));
            }
          });
        })
        .on("error", reject);
    };
    request("https://api.github.com/repos/ohmyjersh/mimic-ai/releases/latest");
  });
}

/**
 * Ensure the mimic binary is available.
 * Returns the path to the binary to use.
 *
 * Resolution order:
 * 1. If user configured a custom binaryPath (not "mimic"), use it as-is
 * 2. Check if "mimic" is on PATH
 * 3. Check if already downloaded to globalStoragePath
 * 4. Download from GitHub Releases
 */
export async function ensureBinary(
  configuredPath: string,
  globalStoragePath: string,
  autoDownload: boolean
): Promise<string> {
  // 1. Custom path configured — use as-is
  if (configuredPath !== "mimic") {
    return configuredPath;
  }

  // 2. Check if mimic is on PATH
  if (await canExecute("mimic")) {
    return "mimic";
  }

  // 3. Check well-known install locations
  for (const candidate of getWellKnownPaths()) {
    if (fs.existsSync(candidate) && await canExecute(candidate)) {
      return candidate;
    }
  }

  // 4. Check if already downloaded
  const assetName = getAssetName();
  if (!assetName) {
    throw new Error(
      `Unsupported platform: ${os.platform()}/${os.arch()}. ` +
        "Please install mimic manually and set mimic.binaryPath."
    );
  }

  const binaryName = os.platform() === "win32" ? "mimic.exe" : "mimic";
  const downloadedPath = path.join(globalStoragePath, binaryName);

  if (fs.existsSync(downloadedPath)) {
    return downloadedPath;
  }

  // 5. Auto-download from GitHub
  if (!autoDownload) {
    throw new Error(
      "mimic binary not found. Install it manually or enable mimic.autoDownload."
    );
  }

  fs.mkdirSync(globalStoragePath, { recursive: true });

  await vscode.window.withProgress(
    {
      location: vscode.ProgressLocation.Notification,
      title: "Mimic: Downloading binary...",
      cancellable: false,
    },
    async () => {
      const release = await fetchLatestRelease();
      const asset = release.assets.find((a) => a.name === assetName);
      if (!asset) {
        throw new Error(
          `No release asset found for ${assetName} in ${release.tag_name}`
        );
      }
      await downloadFile(asset.browser_download_url, downloadedPath);

      if (os.platform() !== "win32") {
        fs.chmodSync(downloadedPath, 0o755);
      }
    }
  );

  vscode.window.showInformationMessage("Mimic: Binary downloaded successfully.");
  return downloadedPath;
}
