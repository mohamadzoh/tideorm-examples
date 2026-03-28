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
    @{ Name = 'schema_file_demo'; Args = @('run', '--bin', 'schema_file_demo') },
    @{ Name = 'validation_demo'; Args = @('run', '--bin', 'validation_demo') },
    @{ Name = 'caching_demo'; Args = @('run', '--bin', 'caching_demo') },
    @{ Name = 'profiling_demo'; Args = @('run', '--bin', 'profiling_demo') },
    @{ Name = 'logging_callbacks_demo'; Args = @('run', '--bin', 'logging_callbacks_demo', '--features', 'sqlite runtime-tokio', '--no-default-features') },
    @{ Name = 'relations_foreign_keys_demo'; Args = @('run', '--bin', 'relations_foreign_keys_demo', '--features', 'sqlite runtime-tokio', '--no-default-features') },
    @{ Name = 'fulltext_demo'; Args = @('run', '--bin', 'fulltext_demo') },
    @{ Name = 'tokenization_demo'; Args = @('run', '--bin', 'tokenization_demo') },
    @{ Name = 'attachments_translations_demo'; Args = @('run', '--bin', 'attachments_translations_demo') },
    @{ Name = 'attachment_url_demo'; Args = @('run', '--bin', 'attachment_url_demo') },
    @{ Name = 'datetime_types_demo'; Args = @('run', '--bin', 'datetime_types_demo') },
    @{ Name = 'seaorm2_features_demo'; Args = @('run', '--bin', 'seaorm2_features_demo') }
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

Write-Host 'All examples completed successfully.'