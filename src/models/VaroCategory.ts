import type { VaroNode } from './VaroNode';
import type { VaroNodeGroup } from './VaroNodeGroup';

export class VaroCategory {
  name: string;
  expanded: boolean;
  nodes: VaroNode[];
  groups: VaroNodeGroup[];

  constructor(data: {
    name: string;
    expanded?: boolean;
    nodes?: VaroNode[];
    groups?: VaroNodeGroup[];
  }) {
    this.name = data.name;
    this.expanded = data.expanded ?? true;
    this.nodes = data.nodes ?? [];
    this.groups = data.groups ?? [];
  }

  get hasVisibleNodes(): boolean {
    return this.nodes.some(node => node.visible);
  }

  get hasVisibleGroupNodes(): boolean {
    return this.groups.some(group => group.visible);
  }
}