library(shiny)
library(jsonlite)

ui <- fluidPage(
  titlePanel("Configuration Generator"),
  fluidRow(
    column(4,
      numericInput("max_age", "Max Age (days)", value = 365 * 12),
      numericInput("max_known_cells", "Max Known Cells", value = 60),
      numericInput("adult_survival_day", "Adult Survival Day", value = 0.9647),
      numericInput("piglet_survival_day", "Piglet Survival Day", value = 0.9438),
      numericInput("min_stay_time", "Min Stay Time (days)", value = 1),
      numericInput("max_stay_time", "Max Stay Time (days)", value = 14),
      numericInput("default_daily_movement_distance", "Default Daily Movement Distance", value = 20)
    ),
    column(4,
      numericInput("good_year_chance", "Good Year Chance (%)", value = 15),
      numericInput("burn_in_period", "Burn-in Period (days)", value = 0),
      numericInput("beta_w", "Beta W", value = 0.05),
      numericInput("beta_b", "Beta B", value = 0.001),
      numericInput("beta_c", "Beta C", value = 0.6),
      numericInput("carcass_contact_prob", "Carcass Contact Probability", value = 0.10),
      numericInput("p_symptomatic", "Probability of Being Symptomatic", value = 0.5),
      numericInput("n_starting_groups", "Number of Starting Groups", value = 10),
      numericInput("seed", "Seed", value = 1234)
    ),
    column(4,
      numericInput("max_hr_cells", "Max HR Cells", value = 2000),
      numericInput("min_hr_cells", "Min HR Cells", value = 1000),
      numericInput("hr_border_fuzzy", "HR Border Fuzzy", value = 0.1),
      numericInput("ap_max_jitter", "AP Max Jitter", value = 6),
      numericInput("ap_jitter_factor", "AP Jitter Factor", value = 2),
      numericInput("min_ap", "Min AP", value = 4),
      numericInput("runtime", "Runtime (days)", value = 365),
      actionButton("save", "Save Configuration"),
      actionButton("close", "Close App")
    )
  ),
  mainPanel(
    verbatimTextOutput("json_output")
  )
)

server <- function(input, output) {
  observeEvent(input$save, {
    config <- list(
      max_age = input$max_age,
      max_known_cells = input$max_known_cells,
      runtime = input$runtime,
      adult_survival_day = input$adult_survival_day,
      piglet_survival_day = input$piglet_survival_day,
      min_stay_time = input$min_stay_time,
      max_stay_time = input$max_stay_time,
      default_daily_movement_distance = input$default_daily_movement_distance,
      good_year_chance = input$good_year_chance,
      burn_in_period = input$burn_in_period,
      beta_w = input$beta_w,
      beta_b = input$beta_b,
      beta_c = input$beta_c,
      carcass_contact_prob = input$carcass_contact_prob,
      p_symptomatic = input$p_symptomatic,
      n_starting_groups = input$n_starting_groups,
      seed = input$seed,
      max_hr_cells = input$max_hr_cells,
      min_hr_cells = input$min_hr_cells,
      hr_border_fuzzy = input$hr_border_fuzzy,
      ap_max_jitter = input$ap_max_jitter,
      ap_jitter_factor = input$ap_jitter_factor,
      min_ap = input$min_ap
    )
    
    write_json(config, "../config.json", auto_unbox = TRUE, pretty = TRUE)
    
    output$json_output <- renderPrint({
      toJSON(config, auto_unbox = TRUE, pretty = TRUE)
    })
  })
  
  observeEvent(input$close, {
    stopApp()
  })
}

shinyApp(ui = ui, server = server)