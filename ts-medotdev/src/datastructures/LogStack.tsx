interface ILogEntry {
  command: string;
  response: React.ReactNode;
}

interface ILogStack {
  push(entry: ILogEntry): void;
  getAll(): ILogEntry[];
}

export class LogStack implements ILogStack {
  private stack: ILogEntry[] = [];

  push(entry: ILogEntry): void {
    this.stack.push(entry);
  }

  clear(): void {
    this.stack = [];
  }

  getAll(): ILogEntry[] {
    return [...this.stack].reverse();
  }
}
