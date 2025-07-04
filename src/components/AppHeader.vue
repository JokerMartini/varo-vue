<script setup lang="ts">
import { getVersion, getName } from "@tauri-apps/api/app";

const appStore = useAppStore();
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
            label: "Developer Debug",
            icon: 'i-lucide-bug',
            onSelect(e: Event) {
              appStore.showDeveloperDialog = true;  
            },
        }, 
    ],
    [
        {
            label: "About",
            icon: "i-lucide-info",
            onSelect(e: Event) {
                appStore.showAboutDialog = true;
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
            checked: appStore.showGroups,
            onSelect(e: Event) {
                appStore.toggleGroups()
            },
        },
        {
            label: "Toggle Categories",
            type: 'checkbox' as const,
            icon: "i-lucide-shapes",
            checked: appStore.showCategories,
            onSelect(e: Event) {
                appStore.toggleCategories()
            },
        },
    ],
    [
        {
            label: "Unhide All Nodes",
            icon: "i-lucide-scan-eye",
            onSelect(e: Event) {
                appStore.unhideAllNodes();
                appStore.unhideAllNodeGroups();
            },
        },
        {
            label: "Toggle Hidden Nodes",
            type: 'checkbox' as const,
            checked: appStore.showHiddenNodes,
            icon: appStore.showHiddenNodes ? "i-lucide-eye" : "i-lucide-eye-off",
            onSelect(e: Event) {
                appStore.toggleHiddenNodeVisibility();
            },
        },
    ],
]);

const doit = () => {
    console.log("here");
    // appStore.launchSomething();
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
                        v-model="appStore.searchQuery"
                        icon="i-lucide-search"
                        placeholder="Search..."
                        class="w-full"
                        variant="soft"
                        @keydown.esc="appStore.searchQuery = ''"
                        :ui="{ trailing: 'pe-1' }"
                    >
                        <template v-if="appStore.searchQuery?.length" #trailing>
                            <UButton
                                color="neutral"
                                variant="link"
                                size="sm"
                                icon="i-lucide-circle-x"
                                aria-label="Clear input"
                                @click="appStore.searchQuery = ''"
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
