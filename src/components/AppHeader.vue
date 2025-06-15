<script setup lang="ts">
import { getVersion, getName } from "@tauri-apps/api/app";

const nodeStore = useVaroNodeStore();
const appName = ref("");
const appVersion = ref("");
const colorMode = useColorMode()

const isDark = computed({
  get() {
    return colorMode.value === 'dark'
  },
  set(_isDark) {
    colorMode.preference = _isDark ? 'dark' : 'light'
  }
})

const mainMenuItems = computed(() => [
    [
       {
            label: "Dark/Light Theme",
            icon: isDark.value ? 'i-lucide-moon' : 'i-lucide-sun',
            // icon: `isDark ? 'i-lucide-moon' : 'i-lucide-sun'" class="shrink-0 size-5"`,
            slot: 'theme' as const, 
            onSelect(e: Event) {
                isDark.value = !isDark.value;
            },
        }, 
    ],
    [
        {
            label: "About",
            icon: "i-lucide-info",
            onSelect(e: Event) {
                nodeStore.showAboutDialog = true;
            },
        },
    ],
]);

const viewMenuItems = computed(() => [
    [
        {
            label: "Toggle Groups",
            type: 'checkbox' as const,
            icon: "i-lucide-layout-list",
            checked: nodeStore.showGroups,
            onSelect(e: Event) {
                nodeStore.toggleGroups()
            },
        },
        {
            label: "Toggle Categories",
            type: 'checkbox' as const,
            icon: "i-lucide-shapes",
            checked: nodeStore.showCategories,
            onSelect(e: Event) {
                nodeStore.toggleCategories()
            },
        },
    ],
    [
        {
            label: "Unhide All Nodes",
            icon: "i-lucide-scan-eye",
            onSelect(e: Event) {
                nodeStore.unhideAllNodes();
                nodeStore.unhideAllNodeGroups();
            },
        },
        {
            label: "Toggle Hidden Nodes",
            type: 'checkbox' as const,
            checked: nodeStore.showHiddenNodes,
            icon: nodeStore.showHiddenNodes ? "i-lucide-eye" : "i-lucide-eye-off",
            onSelect(e: Event) {
                nodeStore.toggleHiddenNodeVisibility();
            },
        },
    ],
]);

const doit = () => {
    console.log("here");
    nodeStore.launchSomething();
};

const openLogs = () => {
    console.log("TODO: Open Log's folder/file");
};

onMounted(async () => {
    appName.value = await getName();
    appVersion.value = await getVersion();
});

const selectedEnv = ref('Default')

</script>

<template>
    <div class="bg-(--ui-bg)/75 backdrop-blur border-b border-(--ui-border) h-14 sticky top-0 z-50 shrink-0">
        <div class="flex items-center justify-between gap-3 h-full px-4">
            
            <!-- left -->
            <div class="flex items-center gap-2">
                <!-- <UTooltip :text="`${appName} - ${appVersion}`">
                    <UIcon name="i-lucide-box" class="shrink-0 size-8" />
                </UTooltip> -->
                <EnvPresetSelect/>
            </div>

            <!-- center -->
            <div class="grow">
                <UButtonGroup class="w-full">
                    <UInput
                        v-model="nodeStore.searchQuery"
                        icon="i-lucide-search"
                        placeholder="Search..."
                        class="w-full"
                        variant="soft"
                        @keydown.esc="nodeStore.searchQuery = ''"
                        :ui="{ trailing: 'pe-1' }"
                    >
                        <template v-if="nodeStore.searchQuery?.length" #trailing>
                            <UButton
                                color="neutral"
                                variant="link"
                                size="sm"
                                icon="i-lucide-circle-x"
                                aria-label="Clear input"
                                @click="nodeStore.searchQuery = ''"
                            />
                        </template>
                    </UInput>

                    <!-- <UTooltip>
                        <UDropdownMenu :items="viewMenuItems" :ui="{ content: 'w-60' }">
                            <UButton icon="i-lucide-settings-2" variant="soft" color="neutral" class="shrink-0"> </UButton>
                        </UDropdownMenu>
                    </UTooltip> -->
                </UButtonGroup>
            </div>

            <!-- right -->
            <div class="flex items-center gap-2">
                <UTooltip>
                    <UDropdownMenu :items="viewMenuItems" :ui="{ content: 'w-60' }">
                        <UButton icon="i-lucide-settings-2" variant="ghost" color="neutral" class="shrink-0"> </UButton>
                    </UDropdownMenu>
                </UTooltip>

                <!-- <UButton @click="doit">Click</UButton> -->
                <!-- <UButton @click="openLogs">Logs</UButton> -->
                
                <UDropdownMenu :items="mainMenuItems">
                    <UButton icon="i-lucide-ellipsis-vertical" variant="ghost" color="neutral" class="shrink-0">
                    </UButton>
                </UDropdownMenu>
            </div>
        </div>
    </div>
</template>
