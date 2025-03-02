import { signal } from "@preact/signals";

export enum APICallStatus {
    Idle,
    Processing,
    Error,
    Ok,
}

export const tableKind = signal<"Geradeturnen" | "Spiraleturnen" | "Sprung">("Geradeturnen");

export const tableValues = signal<PocketPaper>();

export const tableAlteredByParent = signal<boolean>(false);

export const apiCallStatus = signal<APICallStatus>(APICallStatus.Idle);

export const apiCallResponse = signal<{success: boolean, body: string} | null>(null);

export interface MoveEntry {
    isDismount: boolean,
    abbr: string,
    desc: string,
    structureGroups: string,
    difficultyValue: string,
}

export interface PocketPaper {
    discipline: "Geradeturnen" | "Spiraleturnen" | "Sprung",
    moves: Map<string, MoveEntry>,
    fulfilledStructureGroups: Map<number, boolean>,
    finalDifficulty: number,
    highestElements: Map<string, number | null>
}