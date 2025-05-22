<script setup>
const nodeStore = useVaroNodeStore();
</script>
<template>
    <div>
        <div class="grid gap-4 grid-cols-[repeat(auto-fill,minmax(180px,1fr))]">
            <VaroNodeCard v-for="node in nodeStore.filteredNodes" :key="node.id" :node="node" />
        </div>

        <USeparator class="py-6" />

        <div class="grid gap-4 grid-cols-[repeat(auto-fill,minmax(180px,1fr))]">
            <VaroNodeGroupCard v-for="group in nodeStore.filteredNodeGroups" :key="group.id" :group="group" />
        </div>
    </div>
</template>
