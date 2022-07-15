use clap::Parser;

#[derive(Parser, Debug)]
/// Send a metric to datadog via the api
///
/// Uses the datadog API to send either a gauge, counter, or histogram to the datadog api
/// Sends one metric per invocation
/// Requires DD_API_KEY to be set in the environment to function.
#[clap(author, version, about = "Send a metric to datadog via the API.")]
pub struct Args {
    /// Name of the metric to send
    #[clap(short, long)]
    pub name: String,

    /// Type of the metric to send
    #[clap(short, long, default_value = "counter")]
    pub r#type: String,

    /// Metric value
    #[clap(short, long, default_value_t = 1.0)]
    pub value: f64,

    /// Optional tags to add to sent metric in the format key:value,key2:value2
    #[clap(long, default_value = "")]
    pub tags: String,

    /// Optional places to output the metric to as a comma separated list of destinations. Valid destinations are: stdout, api.
    #[clap(short, long, default_value = "api")]
    pub outputs: String,

    /// Will print verbose output to stdout
    #[clap(long)]
    pub verbose: bool,
}
