
export class VaroNode {
    id: string;
    name: string;
    category: string;
    group_id: string;
    icon: string;
    visible?: boolean;
    filepath?: string;
    default_for_group?: boolean;
    description?: string;
    status?: {
      name: string;
      color: string;
    };
    date_modified: number;
  
    constructor(data: {
      id: string;
      name: string;
      category?: string;
      group_id?: string;
      icon?: string;
      visible?: boolean;
      filepath?: string;
      default_for_group?: boolean;
      description?: string;
      status?: {
        name: string;
        color: string;
      };
      date_modified?: number;
    }) {
      this.id = data.id;
      this.name = data.name;
      this.category = data.category ?? 'Uncategorized';
      this.group_id = data.group_id ?? '';
      this.icon = data.icon ?? '';
      this.visible = data.visible ?? true;
      this.filepath = data.filepath ?? '';
      this.default_for_group = data.default_for_group ?? false;
      this.description = data.description;
      this.status = data.status;
      this.date_modified = data.date_modified ?? Date.now();
    }
  }