
// Copyright 2020 TiKV Project Authors. Licensed under Apache-2.0.

define_error_codes!(
    "KV:Engine:",

    ENGINE => ("Engine", "", ""),
    IO => ("IO", "", ""),
    CF_NAME => ("CFName", "", ""),
    DATALOSS => ("DataLoss", "", ""),
    DATACOMPACTED => ("DataCompacted", "", "")
);
