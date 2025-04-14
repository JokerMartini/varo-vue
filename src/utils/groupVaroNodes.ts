import type { VaroNode, VaroNodeGroup } from '~/types/VaroNode';

/**
 * Groups VaroNodes into VaroNodeGroups using their `groupId`.
 */
export function getVaroNodeGroups(nodes: VaroNode[]): VaroNodeGroup[] {
  const groups = new Map<string, VaroNodeGroup>();

  for (const node of nodes) {
    if (!node.groupId) continue;

    if (!groups.has(node.groupId)) {
      groups.set(node.groupId, {
        id: node.groupId,
        name: node.name,
        visible: true,
        nodes: [],
        defaultNodeId: undefined,
      });
    }

    const group = groups.get(node.groupId)!;
    group.nodes.push(node);

    // if (node.defaultForGroup) {
    //   group.defaultNodeId = node;
    // }
  }

  return Array.from(groups.values());
}
