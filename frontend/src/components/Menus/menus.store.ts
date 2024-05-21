import create from 'zustand';
import { useMenus } from '@/components/Menus/menus.queries';
import { IMenuItem } from "@/components/Menus/menus.api";

type MenuStore = {
  menus: IMenuItem[];
  refetch: () => Promise<void>;
  setMenus: (menus: IMenuItem[]) => void;
};

export const useMenuStore = create<MenuStore>((set) => ({
  menus: [],
  refetch: async () => {
    const { menus } = useMenus();
    set({ menus: menus });
  },
  setMenus: (menus) => set({ menus }),
}));
