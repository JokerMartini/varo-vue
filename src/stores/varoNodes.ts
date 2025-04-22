import { defineStore } from 'pinia'
import { getVaroNodeGroups } from '@/utils/groupVaroNodes'
import type { VaroNode } from '~/models/VaroNode'
import { VaroNodeGroup } from '~/models/VaroNodeGroup'
import { VaroCategory } from '~/models/VaroCategory'

export const useVaroNodeStore = defineStore('varoNodes', () => {
  const nodes = ref<VaroNode[]>([])
  const nodeGroups = ref<VaroNodeGroup[]>([])

  function setNodes(newNodes: VaroNode[]) {
    nodes.value = newNodes
    nodeGroups.value = getVaroNodeGroups(newNodes)
  }

  const hiddenUngroupedCount = computed(() =>
    nodes.value.filter(n => !n.visible).length
  )

  const hiddenGroupedCount = computed(() =>
    nodeGroups.value.filter(n => !n.visible).length
  )

  const categories = computed<VaroCategory[]>(() => {
    const groupedByCategory = new Map<string, { groups: VaroNodeGroup[]; nodes: VaroNode[] }>();
  
    for (const group of nodeGroups.value) {
      const category = group.nodes[0]?.category ?? 'Uncategorized';
  
      if (!groupedByCategory.has(category)) {
        groupedByCategory.set(category, { groups: [], nodes: [] });
      }
  
      const entry = groupedByCategory.get(category)!;
      entry.groups.push(group);
      entry.nodes.push(...group.nodes);
    }
  
    return Array.from(groupedByCategory.entries()).map(
      ([name, { groups, nodes }]) =>
        new VaroCategory({
          name,
          expanded: true,
          groups,
          nodes,
        })
    );
  });
  

  return {
    // properties
    nodes,
    nodeGroups,
    categories,
    hiddenUngroupedCount,
    hiddenGroupedCount,

    // methods
    setNodes,
  }
})
