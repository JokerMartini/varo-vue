import { VaroNodeGroup } from '~/models/VaroNodeGroup';
import type { VaroNode } from '~/models/VaroNode';

/**
 * Groups VaroNodes into VaroNodeGroups using `groupId`.
 */
export function getVaroNodeGroups(nodes: VaroNode[]): VaroNodeGroup[] {
  const groupMap = new Map<string, VaroNodeGroup>();

  for (const node of nodes) {
    if (!node.groupId) continue;

    // const groupKey = `${node.groupId}|${node.category}`;
    const groupKey = `${node.groupId}`;

    let group = groupMap.get(groupKey);

    // Create new group if not present
    if (!group) {
      group = new VaroNodeGroup({
        id: node.groupId,
        name: node.name,
        category: node.category ?? 'Uncategorized',
        visible: true,
        nodes: [],
      });
      groupMap.set(groupKey, group);
    }

    group.nodes.push(node);

    // Assign the selected node ID if this one is marked as defaultForGroup
    if (node.defaultForGroup) {
      group.selectedNodeId = node.id;
    }
  }

  // Fallback: if no selectedNodeId is explicitly set, the class handles it in the constructor or getter
  return Array.from(groupMap.values());
}
