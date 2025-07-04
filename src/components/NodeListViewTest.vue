<script setup>
const appStore = useAppStore()
</script>
<template>
    <div>
        <div class="grid gap-4 grid-cols-[repeat(auto-fill,minmax(180px,1fr))]">
            <VaroNodeCard v-for="node in appStore.filteredNodes" :key="node.id" :node="node" />
        </div>

        <USeparator class="py-6" />

        <div class="grid gap-4 grid-cols-[repeat(auto-fill,minmax(180px,1fr))]">
            <VaroNodeGroupCard v-for="group in appStore.filteredNodeGroups" :key="group.id" :group="group" />
        </div>
    </div>
</template>
