// types/VaroNode.ts
export interface VaroVariant {
    id: string;
    name: string;
    description?: string;
    status?: {
      name: string;
      color: string;
      background: string;
    };
    default?: boolean;
  }
  
  export class VaroNode {
    id: string;
    name: string;
    category: string;
    icon: string;
    variants: VaroVariant[];
  
    constructor(data: {
      id: string;
      name: string;
      category: string;
      icon: string;
      variants: VaroVariant[];
    }) {
      this.id = data.id;
      this.name = data.name;
      this.category = data.category;
      this.icon = data.icon;
      this.variants = data.variants;
    }
  }
  