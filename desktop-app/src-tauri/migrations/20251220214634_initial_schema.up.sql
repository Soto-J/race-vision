-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS webview_layout (
        name TEXT PRIMARY KEY,
        x REAL NOT NULL DEFAULT 0,
        y REAL NOT NULL DEFAULT 0,
        width REAL NOT NULL DEFAULT 200,
        height REAL NOT NULL DEFAULT 200
    );

INSERT INTO
    webview_layout (name, x, y, width, height)
VALUES
    ('inputs', 50.0, 50.0, 400.0, 150.0),
    ('standings', 200.0, 200.0, 400.0, 600.0),
    ('track-map', 600.0, 200.0, 300.0, 300.0),
    ('relative', 100.0, 500.0, 400.0, 200.0) ON CONFLICT (name) DO NOTHING;

-- Settings
CREATE TABLE
    IF NOT EXISTS page_settings (
        page TEXT NOT NULL,
        is_active INTEGER NOT NULL DEFAULT 0 CHECK (is_active IN (0, 1)),
        PRIMARY KEY (page, setting)
    );

INSERT INTO
    page_settings,
    is_active
VALUES
    ("inputs",),
    ("relative");

CREATE TABLE
    IF NOT EXISTS app_settings (
        page TEXT PRIMARY KEY,
        use_metric_system INTEGER NOT NULL DEFAULT 0 CHECK (use_metric_system IN (0, 1)),
        startup_overlay_minimized INTEGER NOT NULL DEFAULT 0 CHECK (startup_overlay_minimized IN (0, 1)),
        minimize_to_system_tray INTEGER NOT NULL DEFAULT 0 CHECK (minimize_to_system_tray IN (0, 1)),
        show_race_control_at_startup INTEGER NOT NULL DEFAULT 0 CHECK (show_race_control_at_startup IN (0, 1)),
        use_ctrl_f6_instead_of_f6 INTEGER NOT NULL DEFAULT 0 CHECK (use_ctrl_f6_instead_of_f6 IN (0, 1)),
        use_hardware_acceleration INTEGER NOT NULL DEFAULT 0 CHECK (use_hardware_acceleration IN (0, 1)),
        lower_fps INTEGER NOT NULL DEFAULT 0 CHECK (lower_fps IN (0, 1)),
        show_overlays_in_taskbar INTEGER NOT NULL DEFAULT 0 CHECK (show_overlays_in_taskbar IN (0, 1)),
        vr_compatibility INTEGER NOT NULL DEFAULT 0 CHECK (vr_compatibility IN (0, 1))
    );

CREATE TABLE
    IF NOT EXISTS fuel_calculator_settings (
        page TEXT PRIMARY KEY,
        is_active INTEGER NOT NULL DEFAULT 0 CHECK (is_active IN (0, 1))
    );

CREATE TABLE
    IF NOT EXISTS spotter_settings (
        page TEXT PRIMARY KEY,
        is_active INTEGER NOT NULL DEFAULT 0 CHECK (is_active IN (0, 1))
    );

CREATE TABLE
    IF NOT EXISTS pit_helper_settings (
        page TEXT PRIMARY KEY,
        is_active INTEGER NOT NULL DEFAULT 0 CHECK (is_active IN (0, 1))
    );

CREATE TABLE
    IF NOT EXISTS inputs_settings (
        page TEXT PRIMARY KEY,
        is_active INTEGER NOT NULL DEFAULT 0 CHECK (is_active IN (0, 1)),
        -- Overlay specific
        show_flags INTEGER NOT NULL DEFAULT 0 CHECK (show_flags IN (0, 1)),
        -- General Tab
        in_car INTEGER NOT NULL DEFAULT 0 CHECK (in_car IN (0, 1)),
        out_of_car INTEGER NOT NULL DEFAULT 0 CHECK (out_of_car IN (0, 1)),
        spotting INTEGER NOT NULL DEFAULT 0 CHECK (spotting IN (0, 1)),
        in_garage INTEGER NOT NULL DEFAULT 0 CHECK (in_garage IN (0, 1)),
        -- Content Tab
        rev_lights INTEGER NOT NULL DEFAULT 0 CHECK (rev_lights IN (0, 1)),
        gear_and_speed INTEGER NOT NULL DEFAULT 0 CHECK (gear_and_speed IN (0, 1)),
        abs_activation INTEGER NOT NULL DEFAULT 0 CHECK (abs_activation IN (0, 1)),
        input_bars INTEGER NOT NULL DEFAULT 0 CHECK (input_bars IN (0, 1)),
        steering_wheel INTEGER NOT NULL DEFAULT 0 CHECK (steering_wheel IN (0, 1)),
        boost_ers INTEGER NOT NULL DEFAULT 0 CHECK (boost_ers IN (0, 1)),
        corner_speed INTEGER NOT NULL DEFAULT 0 CHECK (corner_speed IN (0, 1)),
        -- Header Tab
        session_name INTEGER NOT NULL DEFAULT 0 CHECK (session_name IN (0, 1)),
        event_type INTEGER NOT NULL DEFAULT 0 CHECK (event_type IN (0, 1)),
        local_time_24h INTEGER NOT NULL DEFAULT 0 CHECK (local_time_24h IN (0, 1)),
        local_time_am_pm INTEGER NOT NULL DEFAULT 0 CHECK (local_time_am_pm IN (0, 1)),
        in_sim_time_24h INTEGER NOT NULL DEFAULT 0 CHECK (in_sim_time_24h IN (0, 1)),
        in_sim_time_am_pm INTEGER NOT NULL DEFAULT 0 CHECK (in_sim_time_am_pm IN (0, 1)),
        air_temp INTEGER NOT NULL DEFAULT 0 CHECK (air_temp IN (0, 1)),
        track_temp INTEGER NOT NULL DEFAULT 0 CHECK (track_temp IN (0, 1)),
        humidity INTEGER NOT NULL DEFAULT 0 CHECK (humidity IN (0, 1)),
        fog_level INTEGER NOT NULL DEFAULT 0 CHECK (fog_level IN (0, 1)),
        time_remaining INTEGER NOT NULL DEFAULT 0 CHECK (time_remaining IN (0, 1)),
        laps_remaining INTEGER NOT NULL DEFAULT 0 CHECK (laps_remaining IN (0, 1)),
        incident_count INTEGER NOT NULL DEFAULT 0 CHECK (incident_count IN (0, 1)),
        current_lap_time INTEGER NOT NULL DEFAULT 0 CHECK (current_lap_time IN (0, 1)),
        session_best_lap_time INTEGER NOT NULL DEFAULT 0 CHECK (session_best_lap_time IN (0, 1)),
        last_lap_time INTEGER NOT NULL DEFAULT 0 CHECK (last_lap_time IN (0, 1)),
        last_lap_time_calculated INTEGER NOT NULL DEFAULT 0 CHECK (last_lap_time_calculated IN (0, 1)),
        lap_delta_best INTEGER NOT NULL DEFAULT 0 CHECK (lap_delta_best IN (0, 1)),
        lap_delta_optimal INTEGER NOT NULL DEFAULT 0 CHECK (lap_delta_optimal IN (0, 1)),
        lap_delta_session_best INTEGER NOT NULL DEFAULT 0 CHECK (lap_delta_session_best IN (0, 1)),
        lap_delta_session_optimal INTEGER NOT NULL DEFAULT 0 CHECK (lap_delta_session_optimal IN (0, 1)),
        lap_delta_session_last INTEGER NOT NULL DEFAULT 0 CHECK (lap_delta_session_last IN (0, 1)),
        brake_bias INTEGER NOT NULL DEFAULT 0 CHECK (brake_bias IN (0, 1)),
        fuel_level INTEGER NOT NULL DEFAULT 0 CHECK (fuel_level IN (0, 1)),
        water_temp INTEGER NOT NULL DEFAULT 0 CHECK (water_temp IN (0, 1)),
        oil_temp INTEGER NOT NULL DEFAULT 0 CHECK (oil_temp IN (0, 1)),
        sof INTEGER NOT NULL DEFAULT 0 CHECK (sof IN (0, 1)),
        current_stint_in_laps INTEGER NOT NULL DEFAULT 0 CHECK (current_stint_in_laps IN (0, 1)),
        current_stint_in_time INTEGER NOT NULL DEFAULT 0 CHECK (current_stint_in_time IN (0, 1)),
        rpm INTEGER NOT NULL DEFAULT 0 CHECK (rpm IN (0, 1)),
        deploy_mode INTEGER NOT NULL DEFAULT 0 CHECK (deploy_mode IN (0, 1)),
        arb_front INTEGER NOT NULL DEFAULT 0 CHECK (arb_front IN (0, 1)),
        arb_rear INTEGER NOT NULL DEFAULT 0 CHECK (arb_rear IN (0, 1)),
        abs INTEGER NOT NULL DEFAULT 0 CHECK (abs IN (0, 1)),
        tc_1 INTEGER NOT NULL DEFAULT 0 CHECK (tc_1 IN (0, 1)),
        tc_2 INTEGER NOT NULL DEFAULT 0 CHECK (tc_2 IN (0, 1)),
        weight_jacker INTEGER NOT NULL DEFAULT 0 CHECK (weight_jacker IN (0, 1)),
        rear_brake_valve INTEGER NOT NULL DEFAULT 0 CHECK (rear_brake_valve IN (0, 1)),
        precippitation INTEGER NOT NULL DEFAULT 0 CHECK (precippitation IN (0, 1)),
        track_wetness INTEGER NOT NULL DEFAULT 0 CHECK (track_wetness IN (0, 1)),
        weather_declared_wet INTEGER NOT NULL DEFAULT 0 CHECK (weather_declared_wet IN (0, 1)),
        pit_time_loss INTEGER NOT NULL DEFAULT 0 CHECK (pit_time_loss IN (0, 1)),
        wind_direction_for_driver INTEGER NOT NULL DEFAULT 0 CHECK (wind_direction_for_driver IN (0, 1)),
        predicted_position_after_pit_stop INTEGER NOT NULL DEFAULT 0 CHECK (predicted_position_after_pit_stop IN (0, 1)),
        irating_and_gain INTEGER NOT NULL DEFAULT 0 CHECK (irating_and_gain IN (0, 1)),
        push_to_pass INTEGER NOT NULL DEFAULT 0 CHECK (push_to_pass IN (0, 1)),
        -- Footer Tab
        session_name INTEGER NOT NULL DEFAULT 0 CHECK (session_name IN (0, 1)),
        event_type INTEGER NOT NULL DEFAULT 0 CHECK (event_type IN (0, 1)),
        local_time_24h INTEGER NOT NULL DEFAULT 0 CHECK (local_time_24h IN (0, 1)),
        local_time_am_pm INTEGER NOT NULL DEFAULT 0 CHECK (local_time_am_pm IN (0, 1)),
        in_sim_time_24h INTEGER NOT NULL DEFAULT 0 CHECK (in_sim_time_24h IN (0, 1)),
        in_sim_time_am_pm INTEGER NOT NULL DEFAULT 0 CHECK (in_sim_time_am_pm IN (0, 1)),
        air_temp INTEGER NOT NULL DEFAULT 0 CHECK (air_temp IN (0, 1)),
        track_temp INTEGER NOT NULL DEFAULT 0 CHECK (track_temp IN (0, 1)),
        humidity INTEGER NOT NULL DEFAULT 0 CHECK (humidity IN (0, 1)),
        fog_level INTEGER NOT NULL DEFAULT 0 CHECK (fog_level IN (0, 1)),
        time_remaining INTEGER NOT NULL DEFAULT 0 CHECK (time_remaining IN (0, 1)),
        laps_remaining INTEGER NOT NULL DEFAULT 0 CHECK (laps_remaining IN (0, 1)),
        incident_count INTEGER NOT NULL DEFAULT 0 CHECK (incident_count IN (0, 1)),
        current_lap_time INTEGER NOT NULL DEFAULT 0 CHECK (current_lap_time IN (0, 1)),
        session_best_lap_time INTEGER NOT NULL DEFAULT 0 CHECK (session_best_lap_time IN (0, 1)),
        last_lap_time INTEGER NOT NULL DEFAULT 0 CHECK (last_lap_time IN (0, 1)),
        last_lap_time_calculated INTEGER NOT NULL DEFAULT 0 CHECK (last_lap_time_calculated IN (0, 1)),
        lap_delta_best INTEGER NOT NULL DEFAULT 0 CHECK (lap_delta_best IN (0, 1)),
        lap_delta_optimal INTEGER NOT NULL DEFAULT 0 CHECK (lap_delta_optimal IN (0, 1)),
        lap_delta_session_best INTEGER NOT NULL DEFAULT 0 CHECK (lap_delta_session_best IN (0, 1)),
        lap_delta_session_optimal INTEGER NOT NULL DEFAULT 0 CHECK (lap_delta_session_optimal IN (0, 1)),
        lap_delta_session_last INTEGER NOT NULL DEFAULT 0 CHECK (lap_delta_session_last IN (0, 1)),
        brake_bias INTEGER NOT NULL DEFAULT 0 CHECK (brake_bias IN (0, 1)),
        fuel_level INTEGER NOT NULL DEFAULT 0 CHECK (fuel_level IN (0, 1)),
        water_temp INTEGER NOT NULL DEFAULT 0 CHECK (water_temp IN (0, 1)),
        oil_temp INTEGER NOT NULL DEFAULT 0 CHECK (oil_temp IN (0, 1)),
        sof INTEGER NOT NULL DEFAULT 0 CHECK (sof IN (0, 1)),
        current_stint_in_laps INTEGER NOT NULL DEFAULT 0 CHECK (current_stint_in_laps IN (0, 1)),
        current_stint_in_time INTEGER NOT NULL DEFAULT 0 CHECK (current_stint_in_time IN (0, 1)),
        rpm INTEGER NOT NULL DEFAULT 0 CHECK (rpm IN (0, 1)),
        deploy_mode INTEGER NOT NULL DEFAULT 0 CHECK (deploy_mode IN (0, 1)),
        arb_front INTEGER NOT NULL DEFAULT 0 CHECK (arb_front IN (0, 1)),
        arb_rear INTEGER NOT NULL DEFAULT 0 CHECK (arb_rear IN (0, 1)),
        abs INTEGER NOT NULL DEFAULT 0 CHECK (abs IN (0, 1)),
        tc_1 INTEGER NOT NULL DEFAULT 0 CHECK (tc_1 IN (0, 1)),
        tc_2 INTEGER NOT NULL DEFAULT 0 CHECK (tc_2 IN (0, 1)),
        weight_jacker INTEGER NOT NULL DEFAULT 0 CHECK (weight_jacker IN (0, 1)),
        rear_brake_valve INTEGER NOT NULL DEFAULT 0 CHECK (rear_brake_valve IN (0, 1)),
        precippitation INTEGER NOT NULL DEFAULT 0 CHECK (precippitation IN (0, 1)),
        track_wetness INTEGER NOT NULL DEFAULT 0 CHECK (track_wetness IN (0, 1)),
        weather_declared_wet INTEGER NOT NULL DEFAULT 0 CHECK (weather_declared_wet IN (0, 1)),
        pit_time_loss INTEGER NOT NULL DEFAULT 0 CHECK (pit_time_loss IN (0, 1)),
        wind_direction_for_driver INTEGER NOT NULL DEFAULT 0 CHECK (wind_direction_for_driver IN (0, 1)),
        predicted_position_after_pit_stop INTEGER NOT NULL DEFAULT 0 CHECK (predicted_position_after_pit_stop IN (0, 1)),
        irating_and_gain INTEGER NOT NULL DEFAULT 0 CHECK (irating_and_gain IN (0, 1)),
        push_to_pass INTEGER NOT NULL DEFAULT 0 CHECK (push_to_pass IN (0, 1)),
    );

CREATE TABLE
    IF NOT EXISTS inputs_graph_settings (
        page TEXT PRIMARY KEY,
        is_active INTEGER NOT NULL DEFAULT 0 CHECK (is_active IN (0, 1))
    );

CREATE TABLE
    IF NOT EXISTS traffic_indicator_settings (
        page TEXT PRIMARY KEY,
        is_active INTEGER NOT NULL DEFAULT 0 CHECK (is_active IN (0, 1))
    );

CREATE TABLE
    IF NOT EXISTS flat_map_settings (
        page TEXT PRIMARY KEY,
        is_active INTEGER NOT NULL DEFAULT 0 CHECK (is_active IN (0, 1))
    );

CREATE TABLE
    IF NOT EXISTS delta_bar_settings (
        page TEXT PRIMARY KEY,
        is_active INTEGER NOT NULL DEFAULT 0 CHECK (is_active IN (0, 1))
    );

CREATE TABLE
    IF NOT EXISTS track_map_settings (
        page TEXT PRIMARY KEY,
        is_active INTEGER NOT NULL DEFAULT 0 CHECK (is_active IN (0, 1))
    );

CREATE TABLE
    IF NOT EXISTS twitch_chat_settings (
        page TEXT PRIMARY KEY,
        is_active INTEGER NOT NULL DEFAULT 0 CHECK (is_active IN (0, 1))
    );

-- -------------------------------------------