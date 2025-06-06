/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::sync::Arc;

use common::SourceLocationKey;
use fixture_tests::Fixture;
use graphql_ir::Program;
use graphql_ir::build;
use graphql_syntax::parse_executable;
use graphql_test_helpers::diagnostics_to_sorted_string;
use relay_test_schema::TEST_SCHEMA;
use relay_transforms::validate_required_arguments;

pub async fn transform_fixture(fixture: &Fixture<'_>) -> Result<String, String> {
    let source_location = SourceLocationKey::standalone(fixture.file_name);

    let ast = parse_executable(fixture.content, source_location).unwrap();
    let ir = build(&TEST_SCHEMA, &ast.definitions)
        .map_err(|diagnostics| diagnostics_to_sorted_string(fixture.content, &diagnostics))?;

    let program = Program::from_definitions(Arc::clone(&TEST_SCHEMA), ir);
    validate_required_arguments(&program)
        .map_err(|diagnostics| diagnostics_to_sorted_string(fixture.content, &diagnostics))?;

    Ok("OK".to_owned())
}
