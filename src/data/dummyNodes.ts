import { VaroNode } from "~/types/VaroNode";

export const dummyVaroNodes: VaroNode[] = [
  new VaroNode({
    id: "uuid-3dsmax",
    name: "3ds Max",
    category: "3D",
    icon: "./icons/3dsmax.svg",
    variants: [
      {id:"0", name: "3ds Max 2022" },
      {id:"1", name: "3ds Max 2023" },
      {id:"2", name: "3ds Max 2024", default: true, description: "Latest stable version for all artists to use for production", status: { name: "Deprecated", color: "#000000", background: "#fb2c36" } },
      {id:"3", name: "3ds Max 2025", },
      {id:"4", name: "3ds Max 2026", description: "Testing" },
    ],
  }),
  new VaroNode({
    id: "uuid-maya",
    name: "Maya",
    category: "3D",
    icon: "./icons/maya.svg",
    variants: [
      {id:"0", name: "Maya 2020", },
      {id:"1", name: "Maya 2022", },
      {id:"2", name: "Maya 2024", status: { name: "Beta", color: "#000", background: "#ffc107" } },
    ],
  }),
  new VaroNode({
    id: "uuid-blender",
    name: "Blender",
    category: "3D",
    icon: "./icons/blender.svg",
    variants: [
      {id:"0", name: "Blender 2022" },
      {id:"1", name: "Blender 2024", status: { name: "Default", color: "#000", background: "#ffc107" } },
    ],
  }),
  new VaroNode({
    id: "uuid-nuke",
    name: "Nuke",
    category: "Compositing",
    icon: "./icons/nuke.svg",
    variants: [
      {id:"0", name: "Nuke 13.1" },
      {id:"1", name: "Nuke 14 Beta", default: true, status: { name: "Beta", color: "#fff", background: "#f44336" } },
    ],
  }),
  new VaroNode({
    id: "uuid-affinity",
    name: "Affinity Photo",
    category: "2D",
    icon: "./icons/affinity.svg",
    variants: [
      {id:"0", name: "Affinity Photo 2" },
    ],
  }),
  new VaroNode({
    id: "uuid-nim",
    name: "NIM",
    category: "Pipeline",
    icon: "./icons/nim.svg",
    variants: [
      {id:"0", name: "NIM 5.0" },
    ],
  }),
  new VaroNode({
    id: "uuid-nim",
    name: "Documentation",
    category: "Pipeline",
    icon: "./icons/nim.svg",
    variants: [
      {id:"0", name: "Documentation" },
    ],
  }),
];