export class VaroNodeGroup {
  id: string;
  name: string;
  visible: boolean;
  nodes: VaroNode[];
  defaultNodeId?: string;

  constructor(data: {
    id: string;
    name: string;
    visible?: boolean;
    nodes?: VaroNode[];
    defaultNodeId?: string;
  }) {
    this.id = data.id;
    this.name = data.name;
    this.visible = data.visible ?? true;
    this.nodes = data.nodes ?? [];
    this.defaultNodeId = data.defaultNodeId;
  }

  /**
   * Returns the default VaroNode for this group, falling back to first available node.
   */
  // getDefaultNode(): VaroNode | undefined {
  //   if (this.defaultNodeId) {
  //     const found = this.nodes.find(n => n.id === this.defaultNodeId);
  //     if (found) return found;
  //   }
  //   return this.nodes[0];
  // }
}

export class VaroNode {
  id: string;
  name: string;
  category: string;
  groupId: string;
  icon: string;
  visible?: boolean;
  defaultForGroup?: boolean;
  description?: string;
  status?: {
    name: string;
    color: string;
  };

  constructor(data: {
    id: string;
    name: string;
    category?: string;
    groupId?: string;
    icon?: string;
    visible?: boolean;
    defaultForGroup?: boolean;
    description?: string;
    status?: {
      name: string;
      color: string;
    };
  }) {
    this.id = data.id;
    this.name = data.name;
    this.category = data.category ?? 'Uncategorized';
    this.groupId = data.groupId ?? '';
    this.icon = data.icon ?? '';
    this.visible = data.visible ?? true;
    this.defaultForGroup = data.defaultForGroup ?? false;
    this.description = data.description;
    this.status = data.status;
  }
}