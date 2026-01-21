# Change Log

**v5.0.1** - January 21, 2026
- Excluded image from crate to reduce package size.

**v5.0.0** - January 21, 2026

> [!IMPORTANT]
> The machine ID for Linux will change in this version. **Use this version only for new projects or update the device hashes of your current users.**

- Added support for iOS.
- Added an additional source for obtaining the identifier in Linux via `/sys/class/dmi/id/product_uuid`.

**v4.0.0** - October 2, 2025

> [!IMPORTANT]
> `Provisioning UDID` has been [removed](https://github.com/doroved/mid/blob/d6504fe6e10150cd40dbe246ea72058997fbe55b/src/macos.rs#L24) from the dataset for creating Mac device hashes. This happened because when running the application on MacOS Arm through Rosetta 2, the value of `Provisioning UDID` becomes equal to the Hardware UUID, instead of being unique. **Use this version only for new projects or update the device hashes of your current users.**

Run `bash ./test_rosetta.sh` on check_rosetta branch for testing.

**v3.0.7** - August 4, 2025
- Fixed a typo in the function name `run_shell_comand` for Linux and Windows

**v3.0.6** - August 4, 2025
- Added `chip_short` to `mid::additional_data`, which contains the short name of the processor.
```
Example:
chip -> Apple M1 Pro || intel(r) core(tm) i7-4750hq cpu @ 2.00ghz
chip_short -> m1 pro || Intel
```

**v3.0.4** - August 3, 2025
- Changed the function for determining the operating system name

**v3.0.3** - July 17, 2025
- Added `model_name` to `mid::additional_data`

**v3.0.2** - May 3, 2025

> [!NOTE]
> Join the discussion in this [discussion](https://github.com/doroved/mid/discussions/5)

- Added `serde` feature
- Added `PartialEq`, `Eq`, `Clone` attributes for all structures

**v3.0.0** - September 18, 2024

> [!IMPORTANT]
> `Platform ID` has been removed from the mac device hash dataset because after upgrading from macos 14.x to 15.0, it [changed](https://github.com/doroved/mid/blob/d2587cc51f5bf406df7f84ba420e84942b022e23/src/macos.rs#L25), causing the device hash to change. **Use this version only for new projects or update the device hashes of your current users.**

**v2.1.0** - June 30, 2024

- Added `mid::additional_data` function that returns additional device data that is not involved in forming the device hash. Currently available for MacOS only.

**v2.0.0** - March 24, 2024

- Returned `to_lowercase()` for Windows MID result, which was mistakenly removed in v1.1.3. **This will change the current Windows device hashes!** If necessary, use version 2.0.0 for new projects only, or ask users to re-bind the license for new hashes in the current project.
- Added `mid::data` function that returns data structure: key, result, hash.
- `mid::print` outputs data to the console only in debug mode, it will not be included in the release build of the project.
- Linux uses 3 sources to get **machine-id**.
- The secret key for hashing cannot be empty.
- Complete code refactoring has been performed.
