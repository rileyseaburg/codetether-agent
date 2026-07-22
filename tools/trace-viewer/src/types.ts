export interface TraceToken {
  kind: string;
  text: string;
  start: number;
  end: number;
}

export interface ParsedCall {
  record_type: string;
  timestamp?: string;
  syscall?: string;
  arguments?: string;
  result?: string;
  duration?: string | null;
  exit_code?: number;
}

export interface TraceRecord {
  line: number;
  raw: string;
  terminator: string;
  raw_base64: string;
  tokens: TraceToken[];
  parsed: ParsedCall;
}

export interface TraceMetrics {
  records: number;
  bytes: number;
  duration: number;
  span: number;
  sockets: number;
  calls: Map<string, number>;
  first: number;
  last: number;
}
