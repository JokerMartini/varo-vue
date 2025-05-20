import { VaroNode } from "~/models/VaroNode";
import { VaroCategory } from "~/models/VaroCategory";
import { VaroNodeGroup } from "~/models/VaroNodeGroup";

/**
 * Returns a list of unique categories from a set of nodes.
 */
export function getCategoriesFromNodes(nodes: VaroNode[]): VaroCategory[] {
    const seen = new Set<string>();
    const categories: VaroCategory[] = [];

    for (const node of nodes) {
        const categoryName = node.category?.trim() || "Uncategorized";

        if (!seen.has(categoryName)) {
            seen.add(categoryName);
            categories.push(new VaroCategory({ name: categoryName }));
        }
    }

    return categories;
}

/**
 * Groups nodes by only their groupId.
 */
export function getNodeGroups(nodes: VaroNode[]): VaroNodeGroup[] {
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


/**
 * Groups nodes by their category and groupId.
 */
export function getNodeGroupsByCategory(nodes: VaroNode[]): VaroNodeGroup[] {
    const map = new Map<string, VaroNodeGroup>();

    for (const node of nodes) {
        const category = node.category ?? "Uncategorized";
        const groupId = node.groupId ?? "Ungrouped";

        // Combine category and groupId to create a unique key
        const key = `${category}::${groupId}`;

        if (!map.has(key)) {
            map.set(
                key,
                new VaroNodeGroup({
                    id: key,
                    name: groupId,
                    category: category,
                    nodes: [],
                })
            );
        }

        map.get(key)!.addNode(node);
    }

    return Array.from(map.values());
}
