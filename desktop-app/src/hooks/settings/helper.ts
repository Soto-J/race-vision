import type { InputsSettings } from "@/modules/dashboard/inputs/types";

type FeatureTab = "content" | "header" | "footer";

// Extract only keys that have objects with isActive property
type FeatureKey<T extends FeatureTab> = {
  [K in keyof InputsSettings[T]]: InputsSettings[T][K] extends {
    isActive: boolean;
  }
    ? K
    : never;
}[keyof InputsSettings[T]];

// Type for general tab features (direct booleans)
type GeneralFeatureKey =
  | keyof InputsSettings["general"]["showOverlayWhen"]
  | "showFlags";

/**
 * Toggle a feature's isActive property for content/header/footer tabs
 * @param settings - Current InputsSettings
 * @param tab - Tab name (content, header, or footer)
 * @param feature - Feature key within the tab
 * @returns Updated InputsSettings with the feature toggled
 */
export function toggleFeature<T extends FeatureTab>(
  settings: InputsSettings,
  tab: T,
  feature: FeatureKey<T>,
): InputsSettings {
  const currentFeature = settings[tab][feature] as { isActive: boolean };

  return {
    ...settings,
    [tab]: {
      ...settings[tab],
      [feature]: {
        ...currentFeature,
        isActive: !currentFeature.isActive,
      },
    },
  };
}

/**
 * Toggle a feature in the general tab (direct boolean fields)
 * @param settings - Current InputsSettings
 * @param feature - Feature key (showFlags or keys from showOverlayWhen)
 * @returns Updated InputsSettings with the feature toggled
 */
export function toggleGeneralFeature(
  settings: InputsSettings,
  feature: GeneralFeatureKey,
): InputsSettings {
  if (feature === "showFlags") {
    return {
      ...settings,
      general: {
        ...settings.general,
        showFlags: !settings.general.showFlags,
      },
    };
  }

  // Feature is in showOverlayWhen
  const overlayFeature =
    feature as keyof InputsSettings["general"]["showOverlayWhen"];
  return {
    ...settings,
    general: {
      ...settings.general,
      showOverlayWhen: {
        ...settings.general.showOverlayWhen,
        [overlayFeature]: !settings.general.showOverlayWhen[overlayFeature],
      },
    },
  };
}

export type { FeatureTab, FeatureKey, GeneralFeatureKey };

// -------------

// type TabKey = "general" | "content" | "header" | "footer";
// type PageKey = keyof PageSettings;

// export function toggleFeature2(
//   settings: InputsSettings,
//   tab: TabKey,
//   feature: FeatureKey<T>,
// ): InputsSettings {
//   const currentFeature = settings[tab][feature] as { isActive: boolean };

//   return {
//     ...settings,
//     [tab]: {
//       ...settings[tab],
//       [feature]: {
//         ...currentFeature,
//         isActive: !currentFeature.isActive,
//       },
//     },
//   };
// }
