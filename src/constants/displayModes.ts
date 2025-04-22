import type { DisplayMode } from '~/types/DisplayMode';

export interface DisplayModeOption {
  label: string;
  value: DisplayMode;
  description?: string;
}

export const DISPLAY_MODE_OPTIONS: DisplayModeOption[] = [
  {
    label: 'Flat',
    value: 'ungrouped',
    description: 'All apps listed individually without grouping or categorization.',
  },
  {
    label: 'Grouped',
    value: 'grouped',
    description: 'Apps grouped by their group id (e.g., 3ds Max, Nuke, Maya).',
  },
  {
    label: 'Category',
    value: 'category',
    description: 'Apps displayed by domain (e.g., 3D, 2D, Utility).',
  },
  {
    label: 'Category + Grouped',
    value: 'category-grouped',
    description: 'Apps categorized by domain and grouped by version family.',
  },
];
