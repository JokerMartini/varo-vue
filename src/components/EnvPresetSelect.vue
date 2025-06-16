<script setup lang="ts">

const nodeStore = useVaroNodeStore();

const envMenuItems = computed(() => [
    [
        {
            label: "Clear",
            icon: "i-lucide-x",
            onSelect(e: Event) {
                nodeStore.selectedEnvPresetId = "";
            },
        },
    ]
])
</script>

<template>
    <div>
        <UContextMenu :items="envMenuItems">
            <UTooltip text="Environment selector">
                <USelect
                    v-model="nodeStore.selectedEnvPresetId"
                    :items="nodeStore.envPresets"
                    label-key="name"
                    value-key="id"
                    variant="soft"
                    icon="i-lucide-git-branch"
                    placeholder="Select env"
                    class="w-full min-w-36"
                />
            </UTooltip>
        </UContextMenu>
    </div>
</template>