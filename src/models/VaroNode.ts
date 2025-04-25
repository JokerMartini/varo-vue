
export class VaroNode {
    id: string;
    name: string;
    category: string;
    groupId: string;
    icon: string;
    visible?: boolean;
    filepath?: string;
    defaultForGroup?: boolean;
    description?: string;
    status?: {
      name: string;
      color: string;
    };
    dateModified: number;
  
    constructor(data: {
      id: string;
      name: string;
      category?: string;
      groupId?: string;
      icon?: string;
      visible?: boolean;
      filepath?: string;
      defaultForGroup?: boolean;
      description?: string;
      status?: {
        name: string;
        color: string;
      };
      dateModified?: number;
    }) {
      this.id = data.id;
      this.name = data.name;
      this.category = data.category ?? 'Uncategorized';
      this.groupId = data.groupId ?? '';
      this.icon = data.icon ?? '';
      this.visible = data.visible ?? true;
      this.filepath = data.filepath ?? '';
      this.defaultForGroup = data.defaultForGroup ?? false;
      this.description = data.description;
      this.status = data.status;
      this.dateModified = data.dateModified ?? Date.now();
    }
  }