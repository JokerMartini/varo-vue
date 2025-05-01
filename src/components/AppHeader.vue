<script setup lang="ts">

const nodeStore = useVaroNodeStore()

const searchText = ref('');

const showHidden = ref(false)

const items = computed(() => [
    [
        {
            label: 'Show Hidden',
            icon: 'i-lucide-eye',
            type: 'checkbox' as const,
            checked: showHidden.value,
            onSelect(e: Event) {
                // e.preventDefault()
            },
            onUpdateChecked(checked: boolean) {
                showHidden.value = checked
            },
        },
    ],
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

</script>

<template>

    <div class="bg-(--ui-bg)/75 backdrop-blur border-b border-(--ui-border) h-14 sticky top-0 z-50 shrink-0">
        <div class="flex items-center justify-between gap-3 h-full px-4">
            <!-- left -->
            <div class="flex items-center">
                <UTooltip text="Great">
                    <UIcon name="i-lucide-box" class="shrink-0 size-8" />
                </UTooltip>
            </div>
            <!-- center -->
            <div class="grow">
                <UInput 
                    v-model="searchText"
                    icon="i-lucide-search"
                    placeholder="Search..." 
                    class="w-full" 
                    variant="outline"
                    :ui="{ trailing: 'pe-1' }"
                    >
                    <template v-if="searchText?.length" #trailing>
                        <UButton
                            color="neutral"
                            variant="link"
                            size="sm"
                            icon="i-lucide-circle-x"
                            aria-label="Clear input"
                            @click="searchText = ''"
                        />
                    </template>
                </UInput>
            </div>
            <!-- right -->
            <div class="flex items-center">
                <UButton @click="doit">Click</UButton>
                <nuxt-link to="/LogFiles">
                    <UButton>View Logs</UButton>
                </nuxt-link>
                <UDropdownMenu :items="items" :ui="{ content: 'w-60' }">
                    <UButton icon="i-lucide-ellipsis-vertical" 
                    variant="ghost" 
                    color="neutral" class="shrink-0">
                    </UButton>
                </UDropdownMenu>
            </div>
        </div>
    </div>
</template>