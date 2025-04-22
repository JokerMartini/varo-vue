import type { VaroNode } from "./VaroNode";

export class VaroNodeGroup {
  id: string;
  name: string;
  category: string;
  visible: boolean;
  nodes: VaroNode[];
  selectedNodeId?: string;

  constructor(data: {
    id: string;
    name: string;
    category: string;
    visible?: boolean;
    nodes?: VaroNode[];
    selectedNodeId?: string;
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
    return this.nodes.find(n => n.id === this.selectedNodeId) ?? this.nodes[0];
  }

  /**
   * Returns the first node marked defaultForGroup, or fallback to first.
   */
  private getDefaultNodeId(): string {
    return this.nodes.find(n => n.defaultForGroup)?.id ?? this.nodes[0]?.id;
  }

  addNode(node: VaroNode) {
    this.nodes.push(node);
    if (!this.selectedNodeId) {
      this.selectedNodeId = this.getDefaultNodeId();
    }
  }
}