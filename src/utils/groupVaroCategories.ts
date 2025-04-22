import { VaroNode } from '~/models/VaroNode';
import { VaroCategory } from '~/models/VaroCategory';
import { getVaroNodeGroups } from './groupVaroNodes';

export function getVaroCategories(nodes: VaroNode[]): VaroCategory[] {
  const categoryMap = new Map<string, VaroCategory>();

  const nodeGroups = getVaroNodeGroups(nodes);

  for (const node of nodes) {
    if (!categoryMap.has(node.category)) {
      categoryMap.set(node.category, new VaroCategory({ name: node.category }));
    }
    categoryMap.get(node.category)!.addNode(node);
  }

  for (const group of nodeGroups) {
    const cat = categoryMap.get(group.category);
    cat?.addGroup(group);
  }

  return Array
    .from(categoryMap.values())
    .sort((a, b) => a.name.toLowerCase().localeCompare(b.name.toLowerCase()));
}
