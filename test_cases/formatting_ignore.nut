// <sqlsp:ignore_formatting>
local behavior_tree = BehaviorTree()
  .fallback()
    .sequence()
      .condition({ run = retreat_health_check.bindenv(this) })

      .decorator({ run = retreat_retry_decorator.bindenv(this) })
        .action({
          on_init  = retreat_sample_init.bindenv(this),
          on_tick  = retreat_sample_tick.bindenv(this),
          on_abort = retreat_sample_abort.bindenv(this)
        })
      .end()

      .action({
        on_init  = retreat_move_init.bindenv(this),
        on_tick  = retreat_move_tick.bindenv(this),
        on_abort = retreat_move_abort.bindenv(this)
      })

      .decorator({ run = retreat_wait_decorator.bindenv(this) })
        .condition({ run = retreat_health_recovered.bindenv(this) })
      .end()
    .end()    
  .end()
// </sqlsp:ignore_formatting>

local behavior_tree = BehaviorTree()
  .fallback()
    .sequence()
      .condition({ run = retreat_health_check.bindenv(this) })

      .decorator({ run = retreat_retry_decorator.bindenv(this) })
        .action({
          on_init  = retreat_sample_init.bindenv(this),
          on_tick  = retreat_sample_tick.bindenv(this),
          on_abort = retreat_sample_abort.bindenv(this)
        })
      .end()

      .action({
        on_init  = retreat_move_init.bindenv(this),
        on_tick  = retreat_move_tick.bindenv(this),
        on_abort = retreat_move_abort.bindenv(this)
      })

      .decorator({ run = retreat_wait_decorator.bindenv(this) })
        .condition({ run = retreat_health_recovered.bindenv(this) })
      .end()
    .end()    
  .end()
