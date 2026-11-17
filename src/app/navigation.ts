export const navigationItems = [
  {
    id: "overview",
    label: "Overview",
    shortLabel: "OV",
    description: "A high-level view of your developer environment.",
  },
  {
    id: "environment",
    label: "Environment",
    shortLabel: "EN",
    description: "Installed tools, runtimes, versions, and executable locations.",
  },
  {
    id: "repositories",
    label: "Repositories",
    shortLabel: "RE",
    description: "A local catalog of projects and their setup status.",
  },
  {
    id: "requirements",
    label: "Requirements",
    shortLabel: "RQ",
    description: "Project requirements compared with local capabilities.",
  },
  {
    id: "worklist",
    label: "Worklist",
    shortLabel: "WL",
    description: "A user-managed list of planned and pending setup work.",
  },
] as const;

export type PageId = (typeof navigationItems)[number]["id"];

export const initialPage: PageId = "overview";

export function navigationReducer(currentPage: PageId, nextPage: PageId): PageId {
  return currentPage === nextPage ? currentPage : nextPage;
}

export function getPageDefinition(pageId: PageId) {
  return navigationItems.find((item) => item.id === pageId) ?? navigationItems[0];
}
