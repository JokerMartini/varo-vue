<script setup lang="ts">
import type { DropdownMenuItem } from '@nuxt/ui';


const searchText = ref('');

const showGrouping = ref(true)
const showCategories = ref(false)
const showHidden = ref(false)

const items = computed(() => [
    [
        {
            label: 'Enable Grouping',
            icon: 'i-lucide-group',
            type: 'checkbox',
            checked: showGrouping.value,
            onSelect(e: Event) {
                // e.preventDefault()
            },
            onUpdateChecked(checked: boolean) {
                showGrouping.value = checked
            },
        },
        {
            label: 'Enable Categories',
            icon: 'i-lucide-list-tree',
            type: 'checkbox',
            checked: showCategories.value,
            onSelect(e: Event) {
                // e.preventDefault()
            },
            onUpdateChecked(checked: boolean) {
                showCategories.value = checked
            },
            
        },
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

</script>

<template>
    <div class="bg-(--ui-bg)/75 backdrop-blur border-b border-(--ui-border) h-14 sticky top-0 z-50">
        <div class="flex items-center justify-between gap-3 h-full px-4 ">
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
            <div>
                <UDropdownMenu :items="items" :ui="{ content: 'w-48' }">
                    <UButton icon="i-lucide-ellipsis-vertical" 
                    variant="ghost" 
                    color="neutral" class="shrink-0">
                    </UButton>
                </UDropdownMenu>
            </div>
        </div>
    </div>
</template>