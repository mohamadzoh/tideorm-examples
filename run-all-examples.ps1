param(
    [switch]$DryRun
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$examples = @(
    @{ Name = 'basic'; Args = @('run', '--bin', 'basic') },
    @{ Name = 'query_builder'; Args = @('run', '--bin', 'query_builder') },
    @{ Name = 'where_and_or_demo'; Args = @('run', '--bin', 'where_and_or_demo') },
    @{ Name = 'upsert_demo'; Args = @('run', '--bin', 'upsert_demo') },
    @{ Name = 'postgres_demo'; Args = @('run', '--bin', 'postgres_demo') },
    @{ Name = 'postgres_complete'; Args = @('run', '--bin', 'postgres_complete') },
    @{ Name = 'mysql_demo'; Args = @('run', '--bin', 'mysql_demo', '--features', 'mysql runtime-tokio', '--no-default-features') },
    @{ Name = 'sqlite_demo'; Args = @('run', '--bin', 'sqlite_demo', '--features', 'sqlite runtime-tokio', '--no-default-features') },
    @{ Name = 'migrations'; Args = @('run', '--bin', 'migrations') },
    @{ Name = 'migration_test_runner'; Args = @('run', '--bin', 'migration_test_runner') },
    @{ Name = 'schema_file_demo'; Args = @('run', '--bin', 'schema_file_demo') },
    @{ Name = 'auto_model_discovery_demo'; Args = @('run', '--bin', 'auto_model_discovery_demo', '--features', 'sqlite runtime-tokio', '--no-default-features') },
    @{ Name = 'seeding_demo'; Args = @('run', '--bin', 'seeding_demo') },
    @{ Name = 'validation_demo'; Args = @('run', '--bin', 'validation_demo') },
    @{ Name = 'dirty_tracking_demo'; Args = @('run', '--bin', 'dirty_tracking_demo', '--features', 'sqlite runtime-tokio dirty-tracking', '--no-default-features') },
    @{ Name = 'caching_demo'; Args = @('run', '--bin', 'caching_demo') },
    @{ Name = 'profiling_demo'; Args = @('run', '--bin', 'profiling_demo') },
    @{ Name = 'logging_callbacks_demo'; Args = @('run', '--bin', 'logging_callbacks_demo', '--features', 'sqlite runtime-tokio', '--no-default-features') },
    @{ Name = 'relations_foreign_keys_demo'; Args = @('run', '--bin', 'relations_foreign_keys_demo', '--features', 'sqlite runtime-tokio', '--no-default-features') },
    @{ Name = 'entity_manager_demo'; Args = @('run', '--bin', 'entity_manager_demo', '--features', 'sqlite runtime-tokio entity-manager', '--no-default-features') },
    @{ Name = 'fulltext_demo'; Args = @('run', '--bin', 'fulltext_demo') },
    @{ Name = 'encrypted_fields_demo'; Args = @('run', '--bin', 'encrypted_fields_demo', '--features', 'sqlite runtime-tokio encrypted-fields', '--no-default-features') },
    @{ Name = 'tokenization_demo'; Args = @('run', '--bin', 'tokenization_demo') },
    @{ Name = 'attachments_translations_demo'; Args = @('run', '--bin', 'attachments_translations_demo') },
    @{ Name = 'attachment_url_demo'; Args = @('run', '--bin', 'attachment_url_demo') },
    @{ Name = 'datetime_types_demo'; Args = @('run', '--bin', 'datetime_types_demo') },
    @{ Name = 'typed_columns_demo'; Args = @('run', '--bin', 'typed_columns_demo') }
)

function Format-Command {
    param(
        [string[]]$CommandArgs
    )

    $formattedArgs = foreach ($arg in $CommandArgs) {
        if ($arg -match '\s') {
            '"{0}"' -f $arg
        }
        else {
            $arg
        }
    }

    return 'cargo {0}' -f ($formattedArgs -join ' ')
}

# Cargo, `.env` loading and the SQLite demo files all resolve against the current
# directory, so run from this project's root wherever the script is called from.
Push-Location $PSScriptRoot
try {
    foreach ($example in $examples) {
        $commandText = Format-Command -CommandArgs $example.Args
        Write-Host ('=> {0}' -f $commandText)

        if ($DryRun) {
            continue
        }

        & cargo @($example.Args)
        if ($LASTEXITCODE -ne 0) {
            Write-Error ('Example failed: {0}' -f $example.Name)
            exit $LASTEXITCODE
        }
    }
}
finally {
    Pop-Location
}

Write-Host 'All examples completed successfully.'