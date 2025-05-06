<script setup lang="ts">
import { getVersion, getName } from '@tauri-apps/api/app'

const nodeStore = useVaroNodeStore()
const isAboutDialogOpen = ref(false)
const appName = ref('');
const appVersion = ref('');

const items = computed(() => [
    [
        {
            label: 'Settings',
            icon: 'i-lucide-settings',
            onSelect(e: Event) {
            }
        }
    ],
    [
        {
            label: 'About',
            icon: 'i-lucide-info',
            onSelect(e: Event) {
                isAboutDialogOpen.value = true
            }
        }
    ]
])



const doit = () => {
    console.log('here');
    nodeStore.launchSomething();
}

const openLogs = () => {
    console.log("TODO: Open Log's folder/file")
}

onMounted(async () => {
  appName.value = await getName()
  appVersion.value = await getVersion()
})
</script>

<template>

    <div class="bg-(--ui-bg)/75 backdrop-blur border-b border-(--ui-border) h-14 sticky top-0 z-50 shrink-0">
        <div class="flex items-center justify-between gap-3 h-full px-4">
            <!-- left -->
            <div class="flex items-center">
                <UTooltip :text="`${appName} - ${appVersion}`"  >
                    <UIcon name="i-lucide-box" class="shrink-0 size-8" />
                </UTooltip>
            </div>
            <!-- center -->
            <div class="grow">
                <UInput 
                    v-model="nodeStore.searchQuery"
                    icon="i-lucide-search"
                    placeholder="Search..." 
                    class="w-full" 
                    variant="outline"
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
            </div>
            <!-- right -->
            <div class="flex items-center">
                <!-- <UButton @click="doit">Click</UButton> -->
                <!-- <UButton @click="openLogs">View Logs</UButton> -->
                <UDropdownMenu :items="items" :ui="{ content: 'w-60' }">
                    <UButton icon="i-lucide-ellipsis-vertical" 
                    variant="ghost" 
                    color="neutral" class="shrink-0">
                    </UButton>
                </UDropdownMenu>
            </div>
        </div>

        <AboutModal v-model="isAboutDialogOpen" />
        
    </div>
</template>