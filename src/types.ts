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