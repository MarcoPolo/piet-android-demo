package io.marcopolo.pietdemo

import android.content.Context
import android.graphics.*
import android.util.AttributeSet
import android.view.View

class PietDemoView(context: Context, attrs: AttributeSet? = null) : View(context, attrs) {
    external override fun onDraw(canvas: Canvas)
}
