export type DropEffect = "none" | "copy" | "move" | "link";

export interface AppMetadata {
    name: string;
    appPath: string;
    launchArgs: string;
    workingDir: string;
    desc: string;
    iconUrl: string;
}

export interface DnDItem {
    type: "app" | "category";
    name: string;
}

export interface ThemeColorRGB {
    type: "RGB";
    r: number;
    g: number;
    b: number;
}
export interface ThemeColorHSL {
    type: "HSL";
    h: number;
    s: number;
    l: number;
}
export type ThemeColor = ThemeColorRGB | ThemeColorHSL;

export interface ThemeSolid {
    type: "Solid";
    color: ThemeColor;
}
export interface ThemeLinearGradient {
    type: "LinearGradient";
    from: ThemeColor;
    to: ThemeColor;
}
export interface ThemeRadialGradient {
    type: "RadialGradient";
    from: ThemeColor;
    to: ThemeColor;
}
export type Theme = ThemeSolid | ThemeLinearGradient | ThemeRadialGradient;

export interface ConfigBasicInfo {
    headerText: string;
    author: string | null;
    toolboxVersion: [number, number] | null;
    theme: Theme;
}