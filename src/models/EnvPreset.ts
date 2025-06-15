export class EnvVar {
  name: string;
  value: string;
  operation: string;

  constructor(data: { name: string; value: string; operation?: string }) {
    this.name = data.name;
    this.value = data.value;
    this.operation = data.operation ?? 'set';
  }
}

export class EnvPreset {
  id: string;
  name: string;
  description: string;
  env: EnvVar[];

  constructor(data: {
    id: string;
    name: string;
    description?: string;
    env?: { name: string; value: string; operation?: string }[];
  }) {
    this.id = data.id;
    this.name = data.name;
    this.description = data.description ?? '';
    this.env = (data.env ?? []).map(e => new EnvVar(e));
  }

  get isEmpty(): boolean {
    return this.env.length === 0;
  }

  get hasAppendVars(): boolean {
    return this.env.some(e => e.operation === 'append');
  }
}
