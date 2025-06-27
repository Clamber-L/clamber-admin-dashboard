const isPathActive = (currentPath: string, itemPath?: string) => {
    return itemPath && currentPath.startsWith(itemPath);
};
