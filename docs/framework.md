How does the app work:
- App initializes
- Loads config.json file (read config.md for more info) and populates AppState config property
- Loads env presets based on the directories defined in the AppState's config json property 'envPresets/directories'


Process flow not yet figured out how to do elegantly:
- Selects default env preset or first option if available
- We somehow display these env presets on the front end and the user can select which one to load
- Do additional processing based on the selected env preset
