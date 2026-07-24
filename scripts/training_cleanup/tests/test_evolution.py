"""Iceberg-compatible additive schema evolution tests."""

import unittest

from training_cleanup.iceberg_evolution import samples


class Frame:
    """Minimal table frame with known columns."""

    columns = ['run_id', 'source_records']


class Spark:
    """Record table access and schema updates."""

    def __init__(self) -> None:
        self.statements: list[str] = []

    def table(self, name: str) -> Frame:
        self.statements.append(f'TABLE {name}')
        return Frame()

    def sql(self, statement: str) -> None:
        self.statements.append(statement)


class EvolutionTests(unittest.TestCase):
    """Schema discovery avoids the generic catalog partition parser."""

    def test_uses_dataframe_columns_and_adds_only_missing_fields(self) -> None:
        spark = Spark()
        samples(spark, 'iceberg.codetether.training_samples')
        self.assertEqual(
            spark.statements[0], 'TABLE iceberg.codetether.training_samples'
        )
        self.assertEqual(len(spark.statements), 4)
        self.assertNotIn('source_records', ' '.join(spark.statements[1:]))


if __name__ == '__main__':
    unittest.main()
