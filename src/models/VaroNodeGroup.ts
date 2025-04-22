import type { VaroNode } from "./VaroNode";

export class VaroNodeGroup {
  id: string;
  name: string;
  visible: boolean;
  nodes: VaroNode[];
  selectedNodeId?: string;
  category: string;

  constructor(data: {
    id: string;
    name: string;
    visible?: boolean;
    nodes?: VaroNode[];
    selectedNodeId?: string;
    category: string;
  }) {
    this.id = data.id;
    this.name = data.name;
    this.category = data.category;
    this.visible = data.visible ?? true;
    this.nodes = data.nodes ?? [];
    this.selectedNodeId = data.selectedNodeId ?? this.getDefaultNodeId();
  }

  /**
   * Returns the currently selected VaroNode, falling back to default or first node.
   */
  get selectedNode(): VaroNode | undefined {
    if (this.selectedNodeId) {
      const found = this.nodes.find(n => n.id === this.selectedNodeId);
      if (found) return found;
    }
    return this.nodes[0];
  }

  /**
   * Returns the first node marked defaultForGroup, or fallback to first.
   */
  private getDefaultNodeId(): string {
    const defaultNode = this.nodes.find(n => n.defaultForGroup);
    return defaultNode?.id ?? this.nodes[0]?.id;
  }
}