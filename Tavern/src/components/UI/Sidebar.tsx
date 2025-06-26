import React from 'react';
import { useAppStore } from '../../stores/appStore';
import { 
  FolderIcon, 
  UsersIcon, 
  MapIcon, 
  ShieldCheckIcon,
  ChatBubbleLeftIcon,
  CogIcon,
  Bars3Icon,
  XMarkIcon
} from '@heroicons/react/24/outline';

interface SidebarProps {
  collapsed: boolean;
}

const Sidebar: React.FC<SidebarProps> = ({ collapsed }) => {
  const { activePanel, setActivePanel, toggleSidebar } = useAppStore();

  const navItems = [
    { id: 'campaigns', label: 'Campaigns', icon: FolderIcon },
    { id: 'characters', label: 'Characters', icon: UsersIcon },
    { id: 'maps', label: 'Maps', icon: MapIcon },
    { id: 'combat', label: 'Combat', icon: ShieldCheckIcon },
    { id: 'dice', label: 'Dice', icon: '🎲' },
    { id: 'chat', label: 'Chat', icon: ChatBubbleLeftIcon },
    { id: 'settings', label: 'Settings', icon: CogIcon },
  ] as const;

  return (
    <div className={`bg-gray-800 text-white transition-all duration-300 ${
      collapsed ? 'w-16' : 'w-64'
    } flex flex-col`}>
      {/* Header */}
      <div className="p-4 border-b border-gray-700">
        <div className="flex items-center justify-between">
          {!collapsed && (
            <h1 className="text-xl font-bold">D&D VTT</h1>
          )}
          <button
            onClick={toggleSidebar}
            className="p-1 hover:bg-gray-700 rounded"
          >
            {collapsed ? (
              <Bars3Icon className="w-5 h-5" />
            ) : (
              <XMarkIcon className="w-5 h-5" />  
            )}
          </button>
        </div>
      </div>