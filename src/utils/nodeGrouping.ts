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
 * Groups nodes by only their group_id.
 */
export function getNodeGroups(nodes: VaroNode[]): VaroNodeGroup[] {
    const map = new Map<string, VaroNodeGroup>();

    for (const node of nodes) {
        if (!map.has(node.group_id)) {
            map.set(
                node.group_id,
                new VaroNodeGroup({
                    id: node.group_id,
                    name: node.group_id,
                    category: node.category,
                    nodes: [],
                })
            );
        }
        map.get(node.group_id)!.addNode(node);
    }

    return Array.from(map.values());
}


/**
 * Groups nodes by their category and group_id.
 */
export function getNodeGroupsByCategory(nodes: VaroNode[]): VaroNodeGroup[] {
    const map = new Map<string, VaroNodeGroup>();

    for (const node of nodes) {
        const category = node.category ?? "Uncategorized";
        const group_id = node.group_id ?? "Ungrouped";

        // Combine category and group_id to create a unique key
        const key = `${category}::${group_id}`;

        if (!map.has(key)) {
            map.set(
                key,
                new VaroNodeGroup({
                    id: key,
                    name: group_id,
                    category: category,
                    nodes: [],
                })
            );
        }

        map.get(key)!.addNode(node);
    }

    return Array.from(map.values());
}
