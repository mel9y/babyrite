# Security Policy

babyrite Security Policy

## Supported Versions

- Basically, only stable versions are supported
  - Except in exceptional cases, no updates will be released for versions other than the stable version that are no longer supported
  - Also, if the `major` version of stable changes, the major fixed images pushed by release CI will be updated, and any old major version images will not be available

| Version  | Supported | Support Start Date                                                        | Support End Date |
| -------- | --------- | ------------------------------------------------------------------------- | ---------------- |
| v1 | waiting for release | undecided | undecided |

## Reporting a Vulneranily

Security defects should be reported in the appropriate manner, not as an Issue

There are two ways to report. Either way is acceptable

1. Send an email to `me@m2en.dev` signed with the following GPG key
   fingerprint: `78E4 CFE0 B3B2 0C4C 7BAA A3CA 6554 A829 D251 53F9` , [pgp_keys.asc](https://keybase.io/m2en/pgp_keys.asc?fingerprint=78e4cfe0b3b20c4c7baaa3ca6554a829d25153f9)
2. Create security advisories from [GitHub Security Advisories](https://github.com/m2en/babyrite/security/advisories/new)

Please provide the following information in detail for both of the above two methods

- `Impact` - what the vulnerability is and who is affected
- `Process` - the means to reproduce the problem
- `PoC` - Proof of Concept. Actual code (if possible) that demonstrates that an attack using the vulnerability is possible
- `Version` - Version in which the vulnerability exists
