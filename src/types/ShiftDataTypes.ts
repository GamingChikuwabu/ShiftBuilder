export interface ShiftEntry {
    id: string;
    name: string;
    start: string; // "HH:mm"
    end: string;   // "HH:mm"
  }

export interface MemberData {
    name: string;
    role: Role;
  }

export enum Role {
  MANAGER,
  PARTTIME,
}