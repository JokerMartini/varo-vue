import { VaroNode } from '~/models/VaroNode';
import { VaroNodeGroup } from '~/models/VaroNodeGroup';

export function getVaroNodeGroups(nodes: VaroNode[]): VaroNodeGroup[] {
  const map = new Map<string, VaroNodeGroup>();

  for (const node of nodes) {
    if (!map.has(node.groupId)) {
      map.set(
        node.groupId,
        new VaroNodeGroup({
          id: node.groupId,
          name: node.groupId,
          category: node.category,
          nodes: [],
        })
      );
    }
    map.get(node.groupId)!.addNode(node);
  }

  return Array.from(map.values());
}
