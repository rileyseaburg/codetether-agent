# Shared native status fixtures: readiness requires three explicit boolean assertions.
@(
    @{ Name = 'ready'; Json = '{"available":true,"package_identity_present":true,"recognition_probe_succeeded":true}'; Exit = 0; Error = '' },
    @{ Name = 'unavailable'; Json = '{"available":false,"package_identity_present":true,"recognition_probe_succeeded":true}'; Exit = 0; Error = 'OCR_NOT_READY' },
    @{ Name = 'unpackaged'; Json = '{"available":true,"package_identity_present":false,"recognition_probe_succeeded":true}'; Exit = 0; Error = 'OCR_NOT_READY' },
    @{ Name = 'recognition-failed'; Json = '{"available":true,"package_identity_present":true,"recognition_probe_succeeded":false}'; Exit = 0; Error = 'OCR_NOT_READY' },
    @{ Name = 'missing-identity'; Json = '{"available":true,"recognition_probe_succeeded":true}'; Exit = 0; Error = 'OCR_NOT_READY' },
    @{ Name = 'missing-recognition'; Json = '{"available":true,"package_identity_present":true}'; Exit = 0; Error = 'OCR_NOT_READY' },
    @{ Name = 'nonboolean-available'; Json = '{"available":"true","package_identity_present":true,"recognition_probe_succeeded":true}'; Exit = 0; Error = 'OCR_NOT_READY' },
    @{ Name = 'nonboolean-identity'; Json = '{"available":true,"package_identity_present":"true","recognition_probe_succeeded":true}'; Exit = 0; Error = 'OCR_NOT_READY' },
    @{ Name = 'nonboolean-recognition'; Json = '{"available":true,"package_identity_present":true,"recognition_probe_succeeded":"true"}'; Exit = 0; Error = 'OCR_NOT_READY' },
    @{ Name = 'missing'; Json = '{}'; Exit = 0; Error = 'OCR_NOT_READY' },
    @{ Name = 'invalid'; Json = 'not JSON'; Exit = 0; Error = 'OCR_STATUS_INVALID' },
    @{ Name = 'failed'; Json = '{"available":true,"package_identity_present":true,"recognition_probe_succeeded":true}'; Exit = 1; Error = 'OCR_NOT_READY' }
)
