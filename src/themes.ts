import { Theme } from "./types";

export interface ThemePreset {
    name: string;
    theme: Theme;
}

export default [
    {
        name: "Azure",
        theme: {
            type: "LinearGradient",
            from: { type: "RGB", r: 0x28, g: 0x54, b: 0xB5 },
            to: { type: "RGB", r: 0x14, g: 0xC0, b: 0xD3 }
        }
    },
    {
        name: "Violet",
        theme: {
            type: "LinearGradient",
            from: { type: "RGB", r: 0x70, g: 0x28, b: 0xAC },
            to: { type: "RGB", r: 0xAB, g: 0x59, b: 0xC7 }
        }
    },
    {
        name: "Coral",
        theme: {
            type: "LinearGradient",
            from: { type: "RGB", r: 0xBE, g: 0x5F, b: 0x48 },
            to: { type: "RGB", r: 0xE0, g: 0xAA, b: 0x67 }
        }
    },
    {
        name: "Teal",
        theme: {
            type: "LinearGradient",
            from: { type: "RGB", r: 0x00, g: 0x78, b: 0x74 },
            to: { type: "RGB", r: 0x59, g: 0xB1, b: 0xBA }
        }
    },
    {
        name: "Pink",
        theme: {
            type: "LinearGradient",
            from: { type: "RGB", r: 0xB3, g: 0x37, b: 0x7C },
            to: { type: "RGB", r: 0xE2, g: 0x78, b: 0xB1 }
        }
    }
] as ThemePreset[];