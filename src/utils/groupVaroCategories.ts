import { VaroNode } from '~/models/VaroNode';
import { VaroCategory } from '~/models/VaroCategory';
import { VaroNodeGroup } from '~/models/VaroNodeGroup';

export function getVaroCategories(nodes: VaroNode[]): VaroCategory[] {
  const seen = new Set<string>()
  const categories: VaroCategory[] = []

  for (const node of nodes) {
    const categoryName = node.category?.trim() || 'Uncategorized'

    if (!seen.has(categoryName)) {
      seen.add(categoryName)
      categories.push(new VaroCategory({ name: categoryName }))
    }
  }

  return categories
}

export function getNodeGroupsByCategory(nodes: VaroNode[]): Map<string, VaroNodeGroup[]> {
  const result = new Map<string, VaroNodeGroup[]>()

  for (const node of nodes) {
    const category = node.category ?? 'Uncategorized'
    const groupId = node.groupId ?? 'Ungrouped'

    if (!result.has(category)) {
      result.set(category, [])
    }

    const groups = result.get(category)!

    let group = groups.find(g => g.name === groupId)
    if (!group) {
      group = new VaroNodeGroup({
        id: `${category}::${groupId}`,
        name: groupId,
        category: node.category,
      })
      groups.push(group)
    }

    group.nodes.push(node)
  }

  return result
}
