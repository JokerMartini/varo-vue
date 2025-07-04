<script setup lang="ts">
import type { VaroNodeGroup } from '@/models/VaroNodeGroup';
import type { VaroNode } from '@/models/VaroNode';
import type { DropdownMenuItem } from '@nuxt/ui';

const toast = useToast()
const appStore = useAppStore()
const props = defineProps<{ group: VaroNodeGroup }>();
const nodeCard = useTemplateRef('nodeCard')
const nodesMenuOpen = ref(false)
const menuItems = computed<DropdownMenuItem[][]>(() => [
  [
    {
      label: 'Hide',
      disabled: !props.group.visible,
      icon: 'i-lucide-eye-off',
      onSelect(e: Event) {
            props.group.visible = false;
        }
    },
    {
      label: 'Unhide',
      disabled: props.group.visible,
      icon: 'i-lucide-eye',
      onSelect(e: Event) {
        props.group.visible = true;
      }
    },
    {
      label: 'Unhide All',
      icon: 'i-lucide-eye',
      onSelect(e: Event) {
        appStore.unhideAllNodeGroups()
      }
    }
  ],
  [
    {
      label: 'Show in Folder...',
      icon: 'i-lucide-folder',
      onSelect(e: Event) {
        // Show the first node's directory (since all nodes in a group should be in the same directory)
        if (props.group.selectedNodeId) {
          appStore.showNodeInFolder(props.group.selectedNodeId);
        }
      }
    }
  ]
]);

const groupNodeItems = computed<DropdownMenuItem[]>(() => {
  return props.group.nodes.map((node) => ({
    item: node,
    onSelect: (e: Event) => {
        props.group.selectedNodeId = node.id;
        console.log(node);
        console.log(props.group.selectedNodeId);
    }
  }));
});

function handleExecuteClick() {
  nodeCard.value?.classList.add('animate-scale-bounce')
  setTimeout(() => {
    nodeCard.value?.classList.remove('animate-scale-bounce')
  }, 300)

  const node = props.group.selectedNode
  if (node) {
    appStore.executeNode(node.id)
  }
}
</script>

<template>
    <UContextMenu
        :items="menuItems"
    >
        <div 
          ref="nodeCard"
          @click="handleExecuteClick"
          class="
            bg-(--ui-bg-elevated)/75 p-2.5 rounded-[calc(var(--ui-radius)*2)] relative overflow-hidden
            hover:bg-(--ui-bg-elevated)
            active:bg-(--ui-bg-elevated)/50
            cursor-pointer
            "
          :class="{
              'outline-dashed outline-(--ui-text-dimmed) outline-2': !group.visible
          }"
        >
            <!-- content -->
            <div v-if="group.selectedNode" class="flex gap-3 items-start w-full flex-nowrap">
                <!-- Action Button/Icon -->
                <div>
                  <!-- <UIcon name="i-lucide-box" class="shrink-0 size-12"/> -->
                  <IconDisplay :imageData="group.selectedNode.icon" class="shrink-0 size-12"/>
                </div>

                <!-- Title -->
                <div class="flex-grow space-y-1 items-start">

                    <div class="inline-flex gap-2 w-full justify-between items-start">
                      <UTooltip :text="group.selectedNode.description" :disabled="!group.selectedNode.description">
                          <h3 v-if="group.nodes.length === 1" class="font-semibold text-xs">{{ group.selectedNode.name }}</h3>
                          <UDropdownMenu v-else :items="groupNodeItems" v-model:open="nodesMenuOpen">
                              <UButton  
                                  @click.stop
                                  trailing-icon="i-lucide-chevron-down" 
                                  variant="subtle" 
                                  color="neutral" 
                                  class="shrink-0 text-left flex-1 text-xs">
                                  <span class="w-full">
                                      {{ group.selectedNode.name }}
                                  </span>
                              </UButton>

                              <template #item="{ item }">
                                  <VaroNodeGroupOption :node="item.item" @execute="nodesMenuOpen = false"/>
                              </template>

                          </UDropdownMenu>
                      </UTooltip>

                      <!-- Info -->
                      <UPopover mode="hover">
                          <UIcon
                              name="i-lucide-info"
                              class="size-4 text-(--ui-text-dimmed) shrink-0"
                              v-show="group.selectedNode.description"
                          />

                          <template #content>
                              <p class="px-2 py-1 text-xs select-none wrap max-w-64">
                                  {{ group.selectedNode.description }}
                              </p>
                          </template>
                      </UPopover>
                    </div>

                    <!-- Badges -->
                    <div class="flex gap-1">
                      <StatusBadge :status="group.selectedNode.status ?? null" />
                    </div>
                </div>
            </div>
            <div v-else>
                MISSING NODE
            </div>
        </div>
    </UContextMenu>
</template>