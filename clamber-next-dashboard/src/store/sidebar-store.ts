import { create } from 'zustand/react';

interface SidebarStore {
    openKey: string;
    setOpenKey: (key: string) => void;
}

export const sidebarStore = create<SidebarStore>((set) => ({
    openKey: '/',
    setOpenKey: (key) => set({ openKey: key }),
}));
