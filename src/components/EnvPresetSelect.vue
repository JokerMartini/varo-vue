<script setup lang="ts">
import { EnvPreset } from "@/models/EnvPreset";

const nodeStore = useVaroNodeStore();

const envPresets = ref<EnvPreset[]>([
    new EnvPreset({ id: "Dev", name: "Dev", description: "Development preset" }),
    new EnvPreset({ id: "Dev-John", name: "Dev-JohnM", description: "Development preset" }),
    new EnvPreset({ id: "Dev-Kyle", name: "Dev-Kyle", description: "Development preset" }),
    new EnvPreset({ id: "Prod", name: "Prod", description: "Production preset" }),
]);

const selectedPreset = computed(() => nodeStore.envPresets.value.find((p) => p.id === nodeStore.selectedEnvPreset.value) ?? null);

const envMenuItems = computed(() => [
    [
        {
            label: "View Details",
            icon: "i-lucide-external-link",
            onSelect(e: Event) {
                nodeStore.showEnvDialog = true;
            },
        },
        {
            label: "Clear",
            icon: "i-lucide-x",
            onSelect(e: Event) {
                nodeStore.selectedEnvPreset = "";
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
                    v-model="nodeStore.selectedEnvPreset"
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