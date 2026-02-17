import * as vscode from "vscode";
import { ChildProcess, spawn } from "child_process";

interface JsonRpcRequest {
  jsonrpc: "2.0";
  id: number;
  method: string;
  params?: unknown;
}

interface JsonRpcResponse {
  jsonrpc: "2.0";
  id: number;
  result?: unknown;
  error?: { code: number; message: string; data?: unknown };
}

interface PendingRequest {
  resolve: (value: unknown) => void;
  reject: (reason: Error) => void;
}

/**
 * MCP client that communicates with the mimic binary over stdio using JSON-RPC.
 * Messages are newline-delimited JSON.
 */
export class MimicClient {
  private process: ChildProcess | null = null;
  private nextId = 1;
  private pending = new Map<number, PendingRequest>();
  private buffer = "";
  private initialized = false;

  constructor(private binaryPath: string) {}

  /**
   * Spawn the mimic process and perform the MCP initialize handshake.
   */
  async initialize(): Promise<void> {
    if (this.initialized) {
      return;
    }

    this.process = spawn(this.binaryPath, [], {
      stdio: ["pipe", "pipe", "pipe"],
    });

    this.process.stdout!.on("data", (chunk: Buffer) => {
      this.onData(chunk.toString("utf-8"));
    });

    this.process.stderr!.on("data", (chunk: Buffer) => {
      const msg = chunk.toString("utf-8").trim();
      if (msg) {
        console.error(`[mimic stderr] ${msg}`);
      }
    });

    this.process.on("error", (err) => {
      vscode.window.showErrorMessage(
        `Failed to start mimic binary: ${err.message}`
      );
      this.rejectAll(err);
    });

    this.process.on("exit", (code) => {
      if (code !== 0 && code !== null) {
        const err = new Error(`mimic process exited with code ${code}`);
        this.rejectAll(err);
      }
      this.process = null;
      this.initialized = false;
    });

    // MCP initialize handshake
    await this.send("initialize", {
      protocolVersion: "2024-11-05",
      capabilities: {},
      clientInfo: { name: "vscode-mimic", version: "0.1.0" },
    });

    // Send initialized notification (no id, no response expected)
    this.sendNotification("notifications/initialized", {});

    this.initialized = true;
  }

  /**
   * Call an MCP tool by name with the given arguments.
   */
  async callTool(
    name: string,
    args: Record<string, unknown> = {}
  ): Promise<unknown> {
    if (!this.initialized) {
      await this.initialize();
    }
    const result = await this.send("tools/call", { name, arguments: args });
    return result;
  }

  /**
   * Dispose of the client and kill the child process.
   */
  dispose(): void {
    this.pending.forEach(({ reject }) =>
      reject(new Error("Client disposed"))
    );
    this.pending.clear();

    if (this.process) {
      this.process.kill();
      this.process = null;
    }
    this.initialized = false;
  }

  private send(method: string, params?: unknown): Promise<unknown> {
    return new Promise((resolve, reject) => {
      if (!this.process || !this.process.stdin) {
        reject(new Error("mimic process is not running"));
        return;
      }

      const id = this.nextId++;
      const request: JsonRpcRequest = {
        jsonrpc: "2.0",
        id,
        method,
        params,
      };

      this.pending.set(id, { resolve, reject });

      const message = JSON.stringify(request) + "\n";
      this.process.stdin.write(message, "utf-8", (err) => {
        if (err) {
          this.pending.delete(id);
          reject(err);
        }
      });
    });
  }

  private sendNotification(method: string, params?: unknown): void {
    if (!this.process || !this.process.stdin) {
      return;
    }
    const message =
      JSON.stringify({ jsonrpc: "2.0", method, params }) + "\n";
    this.process.stdin.write(message, "utf-8");
  }

  /**
   * Buffer incoming data and process complete newline-delimited JSON messages.
   */
  private onData(data: string): void {
    this.buffer += data;

    let newlineIdx: number;
    while ((newlineIdx = this.buffer.indexOf("\n")) !== -1) {
      const line = this.buffer.slice(0, newlineIdx).trim();
      this.buffer = this.buffer.slice(newlineIdx + 1);

      if (!line) {
        continue;
      }

      let response: JsonRpcResponse;
      try {
        response = JSON.parse(line);
      } catch {
        console.error(`[mimic] Failed to parse JSON-RPC message: ${line}`);
        continue;
      }

      // Ignore notifications (no id)
      if (response.id === undefined || response.id === null) {
        continue;
      }

      const pending = this.pending.get(response.id);
      if (!pending) {
        console.warn(`[mimic] Received response for unknown id: ${response.id}`);
        continue;
      }

      this.pending.delete(response.id);

      if (response.error) {
        pending.reject(
          new Error(`MCP error ${response.error.code}: ${response.error.message}`)
        );
      } else {
        pending.resolve(response.result);
      }
    }
  }

  private rejectAll(err: Error): void {
    this.pending.forEach(({ reject }) => reject(err));
    this.pending.clear();
  }
}
